use crate::papermc::version_json::deserialize_versions;
use serde::Deserialize;
use crate::general::versioning::Versioning;
use crate::minecraft::papermc::version_json::VersionList;
use crate::version::SemanticVersion;

#[derive(Debug, Deserialize)]
pub struct ProjectResponse {
    pub projects: Vec<ProjectEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectEntry {
    pub project: Project,
    #[serde(deserialize_with = "deserialize_versions")]
    pub versions: VersionList,
}

impl Versioning<SemanticVersion> for ProjectEntry {
    fn latest(&self) -> Option<&SemanticVersion> {
        return self.versions.latest()
    }
    fn oldest(&self) -> Option<&SemanticVersion> {
        return self.versions.oldest()
    }
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
}