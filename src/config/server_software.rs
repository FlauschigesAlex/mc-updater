use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ServerSoftware {
    Paper
}

impl Default for ServerSoftware {
    fn default() -> Self {
        Self::Paper
    }
}

pub trait Provider {
    fn get_provider(&self) -> &str;
}

impl Provider for ServerSoftware {
    fn get_provider(&self) -> &str {
        match self {
            ServerSoftware::Paper => "Paper"
        }
    }
}