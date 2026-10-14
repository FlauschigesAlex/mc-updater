pub mod server_software;

use std::fs;
use crate::config::server_software::ServerSoftware;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub server_software: ServerSoftware,
    pub minecraft_version: String,
    pub relative_path: Option<String>,
}

impl Config {
    fn new() -> Self {
        Self {
            server_software: ServerSoftware::default(),
            minecraft_version: "LATEST".to_string(),
            relative_path: None,
        }
    }

    pub fn create_new(path: &Path) -> Result<(Self, File), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directory: {}", e))?;
        }

        log::info!("Creating new config at {}...", path.display());
        let file = File::create(path).map_err(|e| format!("Failed to create config file: {}", e))?;

        let new = Self::new();
        serde_json::to_writer_pretty(&file, &new).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok((new, file))
    }
    
    pub fn load_from_file(path: &Path) -> Result<(Self, File), String> {
        let mut file = File::open(path).map_err(|e| format!("Failed to open config file: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Config = serde_json::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))?;

        Ok((config, file))
    }

    pub fn load_or_create(path: &Path) -> Result<(Self, File), String> {
        Self::load_from_file(path).or_else(|_| Self::create_new(path))
    }
}