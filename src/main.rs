use std::io;

use bambo_processor::BambooProcessor;
use bamboo_client::BambooClient;
use clap::Parser;
use config::{Config, ConfigBuilder, EnvironmentVariables, ProgramArguments};
use days_calculator::{get_eligible_days_this_month, get_working_days_this_month};
use dotenvy::dotenv;

mod bambo_processor;
mod bamboo_client;
mod config;
mod days_calculator;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    let bamboo_client = BambooClient::new(&config);
    let bambo_processor = BambooProcessor::new(&bamboo_client);

    let working_days_this_month = get_working_days_this_month();
    let bank_holidays = bambo_processor.get_bank_holidays().await?;
    let vacation_days = bambo_processor.get_vacation_days().await?;
    let already_added_days = bambo_processor.get_already_added_days().await?;

    let eligible_days = get_eligible_days_this_month(
        &working_days_this_month,
        &vacation_days,
        &already_added_days,
        &bank_holidays,
    );

    if eligible_days.len() == 0 {
        println!("Nothing to do.");
        return Ok(());
    }
    if bank_holidays.len() > 0 {
        println!("(!) Bank holidays this month:");
        for bank_holiday in &bank_holidays {
            println!("{bank_holiday}");
        }
    }
    if vacation_days.len() > 0 {
        println!("(!) You have vacation this month:");
        for vacation_day in &vacation_days {
            println!("{vacation_day}");
        }
    }
    println!("This month you are missing following days:");
    for day in &eligible_days {
        println!("{day}");
    }

    println!(
        "Do you want to send entries to Bamboo for all {} days above? (y/n)",
        eligible_days.len()
    );

    let mut user_response = String::new();
    io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    if user_response.trim().to_lowercase() == "y" {
        let response = &bamboo_client.add_time_entries(&eligible_days).await?;
        if response.status() == 201 {
            println!("Successfully added {} entries!", eligible_days.len());
            return Ok(());
        } else {
            println!("Error adding entries {:#?}", response);
        }
    }

    println!("Do you want to add entries manually one by one? (y/n)");

    let mut user_response = String::new();
    io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read line");

    if user_response.trim().to_lowercase() == "y" {
        for day in &eligible_days {
            println!("Do you want to add timesheet entry on date {}? (y/n)", day);
            let mut response = String::new();

            io::stdin()
                .read_line(&mut response)
                .expect("Failed to read line");

            if response.trim().to_lowercase() == "y" {
                let add_entry_response = &bamboo_client.add_time_entry(day).await?;
                if add_entry_response.status() == 201 {
                    println!("Entry for {} added successfully!", day);
                    println!("");
                } else {
                    println!("Error adding entry for {}: {:#?}", day, add_entry_response)
                }
            } else {
                println!("Skipping adding entry for {}", day);
                println!("");
            }
        }
    } else {
        println!("Fine. Have a nice day!");
    }

    Ok(())
}

fn load_config() -> Result<Config, String> {
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
