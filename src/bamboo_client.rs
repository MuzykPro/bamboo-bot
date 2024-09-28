use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::{
    config::Config, days_calculator::{get_first_day_of_the_month, get_today}, model::{
        add_timesheet_entry::add_entry_request::{AddEntryRequest, AddTimesheetEntry},
        get_timeoff_requests::timeoff_request::TimeOffRequest,
        get_timesheet_entries::timesheet_entry::TimsheetEntry,
        get_whos_out::time_off_entry::TimeOffEntry,
    }
};

pub struct BambooClient<'a> {
    pub config: &'a Config,
    client: reqwest::Client,
}

impl<'a> BambooClient<'a> {
    pub fn new(config: &'a Config) -> Self {
        BambooClient {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_timesheet_entries(
        &self,
    ) -> Result<Vec<TimsheetEntry>, Box<dyn std::error::Error>> {
        let start_of_this_month = get_first_day_of_the_month().format("%Y-%m-%d").to_string();
        let today = get_today().format("%Y-%m-%d").to_string();
        let params = [
            ("start", start_of_this_month),
            ("end", today),
            ("employeeIds", self.config.employee_id.to_string()),
        ];
        let url = format!(
            "{}/api/gateway.php/{}/v1/time_tracking/timesheet_entries",
            self.config.bamboo_base_url, self.config.company
        );
        let url = reqwest::Url::parse_with_params(&url, &params);

        match url {
            Ok(url) => self.get_request(url).await.map(|r| r.unwrap_or_default()),
            Err(e) => {
                println!(
                    "Error while parsing url {:?} with params {:?}: {:?}",
                    &url, &params, &e
                );
                Err(Box::new(e))
            }
        }
    }

    pub async fn add_time_entries(
        &self,
        dates: &Vec<String>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/gateway.php/{}/v1/time_tracking/clock_entries/store",
            self.config.bamboo_base_url, self.config.company
        );

        let body = AddEntryRequest {
            entries: dates
                .into_iter()
                .map(|date| AddTimesheetEntry {
                    employee_id: self.config.employee_id,
                    date: String::from(date),
                    start: "08:00".to_string(),
                    end: "16:00".to_string(),
                })
                .collect(),
        };

        let response = self
            .client
            .post(url)
            .header(String::from("accept"), String::from("application/json"))
            .json(&body)
            .basic_auth(&self.config.api_key, Option::<String>::None)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn add_time_entry(
        &self,
        date: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/gateway.php/{}/v1/time_tracking/clock_entries/store",
            self.config.bamboo_base_url, self.config.company
        );
        let body = AddEntryRequest {
            entries: vec![AddTimesheetEntry {
                employee_id: self.config.employee_id,
                date: String::from(date),
                start: "08:00".to_string(),
                end: "16:00".to_string(),
            }],
        };

        let response = self
            .client
            .post(url)
            .header(String::from("accept"), String::from("application/json"))
            .json(&body)
            .basic_auth(&self.config.api_key, Option::<String>::None)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn get_timeoff_requests(
        &self,
    ) -> Result<Vec<TimeOffRequest>, Box<dyn std::error::Error>> {
        // let two_months_ago = get_first_day_of_the_month() - Months::new(2);
        let start_date = get_first_day_of_the_month().format("%Y-%m-%d").to_string();
        let today = get_today().format("%Y-%m-%d").to_string();

        // let params = [
        //     ("start", start_date),
        //     ("end", today),
        //     ("employeeId", self.config.employee_id.to_string()),
        // ];
        let params = [("start", start_date), ("end", today)];

        let url = format!(
            "{}/api/gateway.php/{}/v1/time_off/requests",
            self.config.bamboo_base_url, self.config.company
        );

        let url = reqwest::Url::parse_with_params(&url, &params);

        match url {
            Ok(url) => self.get_request(url).await.map(|r| r.unwrap_or_default()),
            Err(e) => {
                println!(
                    "Error while parsing url {:?} with params {:?}: {:?}",
                    &url, &params, &e
                );
                Err(Box::new(e))
            }
        }
    }

    pub async fn get_whos_out(&self) -> Result<Vec<TimeOffEntry>, Box<dyn std::error::Error>> {
        let start_of_this_month = get_first_day_of_the_month().format("%Y-%m-%d").to_string();
        let today = get_today().format("%Y-%m-%d").to_string();
        let params = [("start", start_of_this_month), ("end", today)];
        let url = format!(
            "{}/api/gateway.php/{}/v1/time_off/whos_out",
            self.config.bamboo_base_url, self.config.company
        );

        let url = reqwest::Url::parse_with_params(&url, &params);

        match url {
            Ok(url) => self.get_request(url).await.map(|r| r.unwrap_or_default()),
            Err(e) => {
                println!(
                    "Error while parsing url {:?} with params {:?}: {:?}",
                    &url, &params, &e
                );
                Err(Box::new(e))
            }
        }
    }

    async fn get_request<T: DeserializeOwned>(
        &self,
        url: Url,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let response_result = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .basic_auth(&self.config.api_key, Option::<String>::None)
            .send()
            .await;

        let response = match response_result {
            Ok(resp) => resp,
            Err(e) => {
                println!("Error while sending get request: {:?}", e);
                return Err(Box::new(e));
            }
        };

        if !response.status().is_success() {
            return Err(format!(
                "Request failed with status {:?} and error {:?}",
                response.status(),
                response.error_for_status()
            )
            .into());
        }

        let response_text = response.text().await?;

        // println!("{:?}", &response_text);

        match serde_json::from_str(&response_text) {
            Ok(resp_json) => Ok(Option::<T>::Some(resp_json)),
            Err(err) => {
                println!("Error transforming response text to json: {:?}", err);
                return Err(Box::new(err));
            }
        }
    }
}
