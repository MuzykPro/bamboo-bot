use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeOffEntry {
    pub id: i32,
    pub r#type: String,
    pub employee_id: Option<i32>,
    pub name: String,
    pub start: String,
    pub end: String,
}
