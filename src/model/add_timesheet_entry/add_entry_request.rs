use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddEntryRequest {
    pub entries: Vec<AddTimesheetEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddTimesheetEntry {
    pub employee_id: i32,
    pub date: String,
    pub start: String, // hour "09:00"
    pub end: String, // hour "17:00"
}