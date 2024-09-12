use serde::{Deserialize, Serialize};

use super::{project::Project, task::Task};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    pub project: Project,
    pub task: Task,
}