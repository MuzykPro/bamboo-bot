use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestType {
    pub id: String,
    pub name: String,
    pub icon: String,
}