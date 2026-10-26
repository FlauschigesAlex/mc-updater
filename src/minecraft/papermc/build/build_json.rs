use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::minecraft::papermc;
use crate::minecraft::papermc::project_json::ProjectEntry;
use crate::version::SemanticVersion;

#[derive(Debug, Deserialize)]
pub struct BuildEntry {
    pub id: u32,
    pub time: DateTime<Utc>,
    pub channel: BuildChannel,
    pub commits: Vec<BuildCommit>,
    pub downloads: Downloads,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BuildChannel {
    Alpha,
    Beta,
    Stable,
    Recommended,
}

#[derive(Debug, Deserialize)]
pub struct BuildCommit {
    pub sha: String,
    pub time: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    #[serde(rename = "server:default")]
    pub server_jar: FileDownload,
}

#[derive(Debug, Deserialize)]
pub struct FileDownload {
    pub name: String,
    pub size: u64,
    pub url: String,
    pub checksums: HashMap<String, String>,
    #[serde(skip)]
    pub(super) cached_download: Option<Vec<u8>>,
}

#[allow(async_fn_in_trait)]
pub trait Build {
    async fn get_build(&self) -> Option<BuildEntry> {
        self.get_build_versioned(None).await
    }
    async fn get_build_versioned(&self, version: Option<&SemanticVersion>) -> Option<BuildEntry>;
}

impl Build for ProjectEntry {
    async fn get_build_versioned(&self, version: Option<&SemanticVersion>) -> Option<BuildEntry> {
        papermc::get_latest_build(self, version).await
    }
}