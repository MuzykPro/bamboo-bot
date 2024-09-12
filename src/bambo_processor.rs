use crate::{bamboo_client::BambooClient, model::{get_timeoff_requests::timeoff_request::TimeOffRequest, get_whos_out::time_off_entry::TimeOffEntry}};

pub struct BambooProcessor<'a> {
    bamboo_client: &'a BambooClient<'a>
}

impl <'a> BambooProcessor<'a> {
    pub fn new(bamboo_client: &'a BambooClient<'a>) -> Self {
        BambooProcessor {
            bamboo_client
        }
    }

    pub async fn get_bank_holidays(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let whos_out = self.bamboo_client.get_whos_out().await;
        let timeoff_entries: Vec<TimeOffEntry>;
        match whos_out {
            Ok(entries) => timeoff_entries = entries,
            Err(e) => {
                println!("Error while getting who's out: {:?}", e);
                return Err(e);
            }
        }
        let mut bank_holidays = Vec::new();
    
        for entry in timeoff_entries {
            if entry.r#type == "holiday".to_string() {
                if entry.start == entry.end {
                    bank_holidays.push(String::from(&entry.start));
                } else {
                    println!("There is bank holiday from {} to {}, it is not handled automatically!", entry.start, entry.end);
                }
            }
        }
    
        Ok(bank_holidays)
    }
    
    pub async fn get_vacation_days(&self) ->Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self.bamboo_client.get_timeoff_requests().await;
        let vacation_requests: Vec<TimeOffRequest>;
        match response {
            Ok(entries) => vacation_requests = entries,
            Err(e) => {
                println!("Error while getting vacation days: {:?}", e);
                return Err(e);
            }
        }
        let mut vacation_days = Vec::new();
    
        for request in vacation_requests {
            
            for vacation_day in request.dates.unwrap_or_default() {
                if vacation_day.1 == "1".to_string() {
                    vacation_days.push(vacation_day.0);
                }
            }
        }
    
        Ok(vacation_days)
    }

    pub async fn get_already_added_days(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let timesheet_entries = self.bamboo_client.get_timesheet_entries().await;

        match timesheet_entries {
            Ok(entries) => Ok(entries.into_iter().map(|entry| entry.date).collect()),
            Err(e) => {
                println!("Error while getting already added days: {:?}", e);
                Err(e)
            }
        }
    }
}