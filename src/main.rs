#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{env, process::Command};

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    /// Silent mode
    #[clap(short, long, default_value_t = false)]
    silent: bool,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    let client = Client::new();
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());
    let api_addr = format!("{}/completions", config.api_base);
    let response = client
        .post(api_addr)
        .json(&json!({
            "top_p": 1,
            "temperature": 0,
            "max_tokens": 2048,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "stream": false,
            "model": config.model,
            "response_format": {
                "type": "json_object",
            },
            "messages": [
                {
                    "role": "system",
                    "content": config.system_prompt
                },
                {
                    "role": "user",
                    "content": &cli.prompt.join(" ")
                }
            ]
        }))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .send()
        .unwrap();

    let status_code = response.status();
    if status_code.is_client_error() {
        let response_body = response.json::<serde_json::Value>().unwrap();
        let error_message = response_body["error"]["message"].as_str().unwrap();
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("API error: \"{error_message}\"").red().to_string(),
        );
        std::process::exit(1);
    } else if status_code.is_server_error() {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("OpenAI is currently experiencing problems. Status code: {status_code}")
                .red()
                .to_string(),
        );
        std::process::exit(1);
    }

    let content = response.json::<serde_json::Value>().unwrap()["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    let result: serde_json::Value = serde_json::from_str(&content).unwrap();

    if let Some(command) = result["command"].as_str() {
        let command = command.trim();

        if !command.is_empty() {
            if !cli.silent {
                spinner.stop_and_persist(
                    "✔".green().to_string().as_str(),
                    "Got some code!".green().to_string(),
                );

                PrettyPrinter::new()
                    .input_from_bytes(command.as_bytes())
                    .language("bash")
                    .grid(true)
                    .print()
                    .unwrap();
            }

            if !config.post_command.is_empty() {
                Command::new(config.shell)
                    .arg("-c")
                    .arg(config.post_command)
                    .env("PLZ_GENERATED_COMMAND", command)
                    .env(
                        "PLZ_EXECUTABLE",
                        env::current_exe().unwrap().to_str().unwrap(),
                    )
                    .spawn()
                    .unwrap();
            }

            std::process::exit(0);
        }
    }

    let message = result["message"]
        .as_str()
        .unwrap_or("Generate failed")
        .to_string();

    spinner.stop_and_persist(
        "✖".red().to_string().as_str(),
        format!("{message}").red().to_string(),
    );
}
