use std::env;

use clap::{arg, command, Parser};
use derive_builder::Builder;
use dotenvy::dotenv;
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

pub fn load_config() -> Result<Config, String> {
    dotenv().ok();
    let mut config_builder = ConfigBuilder::default();
    let program_args = ProgramArguments::parse();
    let env_config = EnvironmentVariables::load_from_env();
   
    if program_args.api_key.is_none() {
        config_builder.api_key(env_config
            .api_key
            .ok_or("Missing api key. Use --api-key=<API_KEY> or env variable API_KEY=<API_KEY>".to_string())?);
    } else {
        config_builder.api_key(program_args.api_key.unwrap());
    }

    if program_args.company.is_none() {
        config_builder.company(env_config
            .company
            .ok_or("Missing company name. Use --company=<COMPANY_NAME> or env variable COMPANY=<COMPANY_NAME>".to_string())?);
    } else {
        config_builder.company(program_args.company.unwrap());
    }

    if program_args.employee_id.is_none() {
        config_builder.employee_id(env_config
            .employee_id
            .map(|id| id.parse::<i32>().expect("Employee id should be a number"))
            .ok_or("Missing employee id. Use --employee_id=<EMPLOYEE_ID> or env variable EMPLOYEE_ID=<EMPLOYEE_ID>".to_string())?);
    } else {
        config_builder.employee_id(program_args.employee_id.unwrap());
    }

    if let Some(bamboo_url) = program_args.bamboo_base_url {
        config_builder.bamboo_base_url(bamboo_url);
    } else if let Some(bamboo_url) = env_config.bamboo_base_url {
        config_builder.bamboo_base_url(bamboo_url);
    }

   match config_builder.build() {
    Ok(config) => return Ok(config),
    Err(_) => return Err("Error when building config".to_string()),
   }
}
