use crate::general::versioning::Versioning;
use crate::version::SemanticVersion;

pub type MinecraftVersion = String;

trait ToSemanticVersion {
    fn to_semantic_version<'a>(&self, available: &'a Vec<SemanticVersion>) -> Result<&'a SemanticVersion, String>;
}

impl ToSemanticVersion for MinecraftVersion {
    fn to_semantic_version<'a>(&self, available: &'a Vec<SemanticVersion>) -> Result<&'a SemanticVersion, String> {
        if self.to_ascii_lowercase().eq("latest") {
            return available.latest().ok_or_else(
                || "error while fetching latest version".to_string()
            )
        }
        
        available.iter().find(|version| {
            version.to_minecraft_str().eq(self)
        }).ok_or_else(|| "error while fetching version".to_string())
    }
}