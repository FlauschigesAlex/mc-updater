use crate::general::versioning::Versioning;
use crate::version::SemanticVersion;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

pub type VersionList = Vec<SemanticVersion>;

pub fn deserialize_versions<'de, D>(
    deserializer: D,
) -> Result<VersionList, D::Error>
where
    D: Deserializer<'de>,
{
    let groups = HashMap::<String, Vec<SemanticVersion>>::deserialize(deserializer)?;
    let mut list = groups.into_values().flatten().collect::<Vec<SemanticVersion>>();
    list.sort_by(|a, b| b.cmp(a));
    
    Ok(list)
}

impl Versioning<SemanticVersion> for VersionList {
    fn latest(&self) -> Option<&SemanticVersion> {
        return self.iter().max()
    }

    fn oldest(&self) -> Option<&SemanticVersion> {
        return self.iter().min()
    }
}