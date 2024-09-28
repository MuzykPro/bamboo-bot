use std::env;

use clap::{arg, command, Parser};
use derive_builder::Builder;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProgramArguments {

    /// BambooHR API key
    #[arg(short, long)]
    pub api_key: Option<String>,

    /// BambooHR employee id
    #[arg(short, long)]
    pub employee_id: Option<i32>,

    /// Company name
    #[arg(short, long)]
    pub company: Option<String>,

    /// BambooHR base url (optional)
    #[arg(short, long)]
    pub bamboo_base_url: Option<String>,
}

#[derive(Builder)]
pub struct Config {
    pub api_key: String,
    pub employee_id: i32,
    pub company: String,
    #[builder(default = "String::from(\"https://api.bamboohr.com\")")]
    pub bamboo_base_url: String,
}

#[derive(Deserialize, Debug)]

pub struct EnvironmentVariables {
    pub api_key: Option<String>,
    pub employee_id: Option<String>,
    pub company: Option<String>,
    pub bamboo_base_url: Option<String>,
}

impl EnvironmentVariables {
    pub fn load_from_env() -> Self {
        EnvironmentVariables {
            api_key: env::var("API_KEY").ok(),
            employee_id: env::var("EMPLOYEE_ID").ok(),
            company: env::var("COMPANY").ok(),
            bamboo_base_url: env::var("BAMBOO_BASE_URL").ok(),
        }
    }
}
