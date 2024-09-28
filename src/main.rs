use std::io;

use bambo_processor::BambooProcessor;
use bamboo_client::BambooClient;
use days_calculator::{get_eligible_days_this_month, get_weekday, get_working_days_this_month};
use itertools::Itertools;

mod bambo_processor;
mod bamboo_client;
mod config;
mod days_calculator;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    
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
        for bank_holiday in bank_holidays.iter().sorted() {
            println!("{bank_holiday} ({})", get_weekday(bank_holiday)?);
        }
    }
    if vacation_days.len() > 0 {
        println!("(!) You have vacation this month:");
        for vacation_day in vacation_days.iter().sorted() {
            println!("{vacation_day} ({})", get_weekday(&vacation_day)?);
        }
    }
    println!("This month you are missing following days:");
    for day in eligible_days.iter().sorted() {
        println!("{day} ({})", get_weekday(day)?);
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
