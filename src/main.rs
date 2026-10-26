use std::ops::Add;
use std::path::Path;
use crate::config::Config;
use crate::general::file::sha256;
use crate::minecraft::papermc;
use crate::minecraft::papermc::build::build_json::Build;

pub mod request;
pub mod minecraft;
pub mod version;
pub mod general;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
        
    log::info!("");
    log::info!("Initializing server updater...");
    log::info!("");
    
    log::info!("Loading configuration...");
    let path = Path::new("./config.json");
    let (config, _) = Config::load_or_create(&path).unwrap();
    let name = format!("{:?}", config.server_software);
    
    let projects = papermc::check_projects().await.unwrap_or(vec![]);
    let entry = projects.iter().find(|it| {
        it.project.name.eq_ignore_ascii_case(&name)
    }).unwrap_or_else(|| { panic!("Project with name {name} not found.") });
    
    let mut build = entry.get_build().await.unwrap();

    let mut paths = config.relative_path;
    if paths.is_empty() { 
        paths.push("./".to_string())
    }
    
    let mut to_update: Vec<String> = vec![];
    
    for path in paths {
        let file_path = path
            .trim_end_matches("/")
            .to_string()
            .add(&format!("/{}.jar", name.to_ascii_lowercase()));

        let sha256 = sha256(&file_path).unwrap_or_default();
        let Some(remote_sha256) = build.downloads.server_jar.checksums.get("sha256") else {
            log::error!("An error occurred checking {}", file_path);
            return;
        };

        if sha256.eq(remote_sha256) {
            log::info!("{} is already up to date, skipping...", file_path);
            continue;
        }
        
        to_update.push(file_path);
    }
    
    if to_update.is_empty() {
        log::info!("All server jars are up to date!");
        return;
    }

    log::info!("");
    log::info!("Updating {} outdated server jars...", to_update.len());
    log::info!("");

    for path in &to_update {
        if let Err(e) = build.downloads.server_jar.download_with_name(&path).await {
            log::error!("Failed to update server jar at {}: {}", path, e);
        }
        log::info!("Updated {} to {}.", path, build.downloads.server_jar.name);
    }

    log::info!("");
    log::info!("Updated {} outdated server jars.", to_update.len());
}
