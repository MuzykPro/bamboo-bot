use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions {
    pub view: bool,
    pub edit: bool,
    pub cancel: bool,
    pub approve: bool,
    pub deny: bool,
    pub bypass: bool,
}