use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notes {
    pub employee: Option<String>,
    pub manager: Option<String>,
}