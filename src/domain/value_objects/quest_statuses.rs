use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatuses {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

//finding a way to derive string from this enum

impl fmt::Display for QuestStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestStatuses::Open => write!(f, "Open"),
            QuestStatuses::InJourney => write!(f, "InJourney"),
            QuestStatuses::Completed => write!(f, "Completed"),
            QuestStatuses::Failed => write!(f, "Failed"),
        }
    }
}
