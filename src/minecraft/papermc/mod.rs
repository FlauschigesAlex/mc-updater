pub mod build;
pub mod project_json;
mod version_json;

use crate::general::versioning::Versioning;
use crate::minecraft::papermc::build::build_json::BuildEntry;
use crate::minecraft::papermc::project_json::{ProjectEntry, ProjectResponse};
use crate::request::get_request_str;
use crate::version::SemanticVersion;

pub async fn check_projects() -> Option<Vec<ProjectEntry>> {
    let body = get_request_str("https://fill.papermc.io/v3/projects").await
        .inspect_err(|err| eprintln!("Error fetching projects: {}", err))
        .ok()?;

    let response = serde_json::from_str::<ProjectResponse>(&body)
        .inspect_err(|err| eprintln!("Error deserializing projects: {}", err))
        .ok()?;
    
    return Some(response.projects)
}

pub async fn get_latest_build(project: &ProjectEntry, version: Option<&SemanticVersion>) -> Option<BuildEntry> {
    let latest = project.latest()?;
    get_latest_build_raw(&project.project.id, latest, version).await
}

pub async fn get_latest_build_raw(project: &str, latest_version: &SemanticVersion, version: Option<&SemanticVersion>) -> Option<BuildEntry> {
    let version = version.unwrap_or(latest_version);
    
    let body = get_request_str(format!("https://fill.papermc.io/v3/projects/{}/versions/{}/builds/latest", project, version.to_minecraft_str())).await
        .inspect_err(|err| eprintln!("Error fetching projects: {}", err))
        .ok()?;

    let response = serde_json::from_str::<BuildEntry>(&body)
        .inspect_err(|err| eprintln!("Error deserializing projects: {}", err))
        .ok()?;
    
    return Some(response)
}