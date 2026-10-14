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
    log::info!("Initializing server updater...");
    let path = Path::new("./config.json");
    let (config, _) = Config::load_or_create(&path).unwrap();
    let name = format!("{:?}", config.server_software);
    
    let projects = papermc::check_projects().await.unwrap_or(vec![]);
    let entry = projects.iter().find(|it| {
        it.project.name.eq_ignore_ascii_case(&name)
    }).unwrap_or_else(|| { panic!("Project with name ${name} not found.") });
    
    let build = entry.get_build().await.unwrap();
    let path: &str = &config.relative_path
        .unwrap_or(path.parent().unwrap().to_str().unwrap().to_string())
        .trim_end_matches("/")
        .to_string()
        .add(&format!("/{}.jar", name.to_ascii_lowercase()));
    
    let sha256 = sha256(path).unwrap_or_default();
    let remote_sha256 = build.downloads.server_jar.checksums.get("sha256").unwrap();
    
    if sha256.eq(remote_sha256) {
        log::info!("Server jar is already up to date.");
        return;
    }

    log::info!("Downloading new server jar...");
    log::info!("Build: {} | Hash: {sha256}", build.time);
    build.downloads.server_jar.download_with_name(path).await.unwrap();

    log::info!("Finished!");
    return;
}
