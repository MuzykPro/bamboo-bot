use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{actions::Actions, amount::Amount, notes::Notes, request_type::RequestType, status::Status};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeOffRequest {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub status: Status,
    pub start: String,
    pub end: String,
    pub created: String,
    pub r#type: RequestType,
    pub amount: Amount,
    pub actions: Option<Actions>,
    pub dates: Option<HashMap<String, String>>, // This field is optional as per the comment
    pub notes: Option<Notes>,
}




