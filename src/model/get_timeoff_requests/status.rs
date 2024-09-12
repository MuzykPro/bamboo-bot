use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    last_changed: String,
    last_changed_by_user_id: String,
    status: String,
}