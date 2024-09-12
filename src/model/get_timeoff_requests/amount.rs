use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub unit: String,
    pub amount: String,
}