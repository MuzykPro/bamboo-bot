use chrono::{DateTime, Datelike, Local, NaiveDate, Weekday};

pub fn get_eligible_days_this_month(
    working_days: &Vec<String>, 
    vacation_days: &Vec<String>,
    already_added_days: &Vec<String>,
    bank_holidays: &Vec<String>) -> Vec<String> {

    let mut eligible_days = Vec::new();

    for day in working_days {
        if vacation_days.contains(day)
        || already_added_days.contains(day)
        || bank_holidays.contains(day) {
            continue;
        }
        eligible_days.push(String::from(day));
    }
    eligible_days
}

pub fn get_today() -> DateTime<Local> {
    Local::now()
}

pub fn get_first_day_of_the_month() -> DateTime<Local> {
    let today = get_today();
    today.with_day(1).unwrap()
}

pub fn get_working_days_this_month() -> Vec<String> {
    let today = get_today();

    let mut working_days = Vec::new();

    for i in 1..=today.day() {
        let day = today.with_day(i).unwrap();
        let day_of_week = day.naive_local().weekday();
        if day_of_week != Weekday::Sat && day_of_week != Weekday::Sun {
            working_days.push(day.format("%Y-%m-%d").to_string());
        }
    }
    working_days
}

pub fn get_weekday(date_str: &str) -> Result<String, chrono::ParseError> {
    // Parse the date string into a NaiveDate object
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    // Format the date to get the full weekday name (e.g., "Monday")
    let weekday = date.format("%A").to_string();
    // Return the weekday name
    Ok(weekday)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_working_days_this_month() {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let working_days = get_working_days_this_month();
        for day in &working_days {
            dbg!("{}", day);
        }

        assert_eq!(working_days.contains(&today), true);

    }

    #[test]
    fn test_get_eligible_days_this_month() {
        let working_days = vec![
            "2024-08-01", "2024-08-02", "2024-08-05", "2024-08-06", 
            "2024-08-07", "2024-08-08", "2024-08-09", "2024-08-12", 
            "2024-08-13", "2024-08-14", "2024-08-15", "2024-08-16", 
            "2024-08-19", "2024-08-20", "2024-08-21", "2024-08-22", 
            "2024-08-23", "2024-08-26", "2024-08-27", "2024-08-28", 
            "2024-08-29", "2024-08-30"
        ].into_iter().map(|s| s.to_string()).collect();
        let vacation_days = vec!["2024-08-19", "2024-08-20", "2024-08-21", "2024-08-22"].into_iter().map(|s| s.to_string()).collect();
        let already_added_days = vec!["2024-08-01", "2024-08-02", "2024-08-05", "2024-08-06", 
            "2024-08-07", "2024-08-08", "2024-08-09", "2024-08-12"].into_iter().map(|s| s.to_string()).collect();
        let bank_holidays = vec!["2024-08-29", "2024-08-30"].into_iter().map(|s| s.to_string()).collect();
        
        let eligible_days = get_eligible_days_this_month(&working_days, &vacation_days, &already_added_days, &bank_holidays);

        let expected_days: Vec<String> = vec![ 
            "2024-08-13", "2024-08-14", "2024-08-15", "2024-08-16", 
            "2024-08-23", "2024-08-26", "2024-08-27", "2024-08-28", 
        ].into_iter().map(|s| s.to_string()).collect();

        assert_eq!(expected_days, eligible_days);
    }

    #[test]
    fn test_get_weekday() {
        let day = "2024-09-23";
        let weekday = get_weekday(day).expect("Error while extracting weekday");

        assert_eq!(weekday, "Monday".to_string());
    }

}