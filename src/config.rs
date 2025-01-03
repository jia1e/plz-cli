use colored::Colorize;
use std::{env, process::exit};

pub struct Config {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
    pub system_prompt: String,
    pub post_command: String,
    pub shell: String,
}

static DEFAULT_OPENAI_API_BASE: &str = "https://api.openai.com/v1";
static DEFAULT_MODEL: &str = "gpt-3.5-turbo-instruct";
static DEFAULT_SYSTEM_PROMPT: &str = include_str!("system_prompt.md");

impl Config {
    pub fn new() -> Self {
        let api_key = env::var("PLZ_OPENAI_API_KEY").unwrap_or_else(|_| {
            println!("{}", "This program requires an OpenAI API key to run. Please set the PLZ_OPENAI_API_KEY environment variable. https://github.com/m1guelpf/plz-cli#usage".red());
            exit(1);
        });
        let api_base = env::var("PLZ_OPENAI_API_BASE")
            .unwrap_or_else(|_| String::from(DEFAULT_OPENAI_API_BASE));
        let model = env::var("PLZ_OPENAI_MODEL").unwrap_or_else(|_| String::from(DEFAULT_MODEL));
        let system_lang = env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_string());
        let lang = env::var("PLZ_LANG").unwrap_or_else(|_| system_lang);
        let system_prompt =
            env::var("PLZ_SYSTEM_PROMPT").unwrap_or_else(|_| String::from(DEFAULT_SYSTEM_PROMPT));
        let system_prompt = system_prompt.replace("{{LANG}}", &lang);
        let post_command = env::var("PLZ_POST_COMMAND").unwrap_or_else(|_| String::from(""));
        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash"));

        Self {
            api_key,
            api_base,
            model,
            system_prompt,
            post_command,
            shell,
        }
    }
}
