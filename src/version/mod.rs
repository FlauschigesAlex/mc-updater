use std::cmp::Ordering;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Formatter};
use crate::general::alias::Alias;
use crate::version::semantic_version_type::SemanticVersionType;

mod semantic_version_type;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub suffix: String,
    pub version_type: SemanticVersionType,
    raw_value: Option<String>
}

impl SemanticVersion {
    pub(crate) fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            suffix: String::new(),
            version_type: SemanticVersionType::default(),
            raw_value: None
        }
    }
    
    fn new_with_suffix(major: u32, minor: u32, patch: u32, suffix: String) -> Self {
        Self {
            version_type: SemanticVersion::version_type(&suffix),
            major,
            minor,
            patch,
            suffix,
            raw_value: None
        }
    }
    
    fn new_with_suffix_raw(major: u32, minor: u32, patch: u32, suffix: String, raw: String) -> Self {
        Self {
            version_type: SemanticVersion::version_type(&suffix),
            major,
            minor,
            patch,
            suffix,
            raw_value: Some(raw)
        }
    }

    pub(crate) fn parse_from_string(string: String) -> Result<Self, VersionParseError> {
        let (number, suffix) = match string.split_once('-') {
            Some((number, suffix)) => (number, suffix.to_string()),
            None => (string.as_str(), String::new()),
        };

        let mut parts = number.split('.');
        
        let major = parts
            .next()
            .ok_or("Missing major version")?
            .parse::<u32>()
            .map_err(|err| err.to_string())?;

        let minor = parts
            .next()
            .ok_or("Missing minor version")?
            .parse::<u32>()
            .map_err(|err| err.to_string())?;

        let patch = parts
            .next()
            .unwrap_or("0")
            .parse::<u32>()
            .map_err(|err| err.to_string())?;

        Ok(Self::new_with_suffix_raw(major, minor, patch, suffix, string))
    }
    
    pub fn to_string(&self) -> String {
        if self.suffix.is_empty() {
            return format!("{}.{}.{}", self.major, self.minor, self.patch)
        }
        
        return format!("{}.{}.{}-{}", self.major, self.minor, self.patch, self.suffix)
    }
    
    pub fn to_minecraft_str(&self) -> String {
        self.raw_value.clone().unwrap_or(self.to_string().replace(".0", ""))
    }
    
    fn version_type(suffix: &String) -> SemanticVersionType {
        SemanticVersionType::values().find(|version_type| {
            suffix.to_ascii_lowercase().as_str().contains(version_type.name().to_ascii_lowercase().as_str()) || 
                version_type.get_aliases().iter().any(|alias| {
                    suffix.to_ascii_lowercase().as_str().contains(alias.to_ascii_lowercase().as_str())
                })
        }).unwrap_or(SemanticVersionType::Release)
    }
}

pub type VersionParseError = String;

impl Debug for SemanticVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let raw = String::deserialize(deserializer)?;
        SemanticVersion::parse_from_string(raw).map_err(|err| D::Error::custom(err))
    }
}

impl Ord for SemanticVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major.cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
            .then(self.version_type.cmp(&other.version_type))
            .then(self.suffix.cmp(&other.suffix))
    }
}