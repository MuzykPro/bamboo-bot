use serde::{Deserialize, Serialize};

use super::project_info::ProjectInfo;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimsheetEntry {
    pub id: u32,
    pub employee_id: i32,
    pub r#type: String, // `type` is a reserved keyword in Rust, so you need to escape it using `r#`.
    pub date: String,
    pub start: String,
    pub end: String,
    pub timezone: String,
    pub hours: u32,
    pub note: Option<String>,
    pub project_info: Option<ProjectInfo>,
    pub approved_at: Option<String>,
    pub approved: Option<bool>,
}