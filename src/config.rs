use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {

    /// BambooHR API key
    #[arg(short, long)]
    pub api_key: String,

    /// BambooHR employee id
    #[arg(short, long)]
    pub employee_id: i32,

    /// Company name
    #[arg(short, long)]
    pub company: String,

    /// BambooHR base url (optional)
    #[arg(short, long, default_value_t = String::from("https://api.bamboohr.com"))]
    pub bamboo_base_url: String,
}
