use crate::general::alias::Alias;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticVersionType {
    Alpha,
    Beta,
    ReleaseCandidate,
    Snapshot,
    Release
}

impl SemanticVersionType {
    pub const VALUES: [Self; 5] = [
        Self::Alpha,
        Self::Beta,
        Self::ReleaseCandidate,
        Self::Snapshot,
        Self::Release,
    ];

    pub fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Alpha => "Alpha",
            Self::Beta => "Beta",
            Self::ReleaseCandidate => "Release Candidate",
            Self::Snapshot => "Snapshot",
            Self::Release => "Release",
        }
    }
    
    pub fn is_stable(&self) -> bool {
        match self {
            SemanticVersionType::Release | SemanticVersionType::Snapshot => true,
            _ => false,
        }
    }
}

impl Alias for SemanticVersionType {
    fn get_aliases(&self) -> Vec<&str> {
        match self {
            SemanticVersionType::Alpha => vec!["alpha", "a"],
            SemanticVersionType::Beta => vec!["beta", "b"],
            SemanticVersionType::ReleaseCandidate => vec!["release-candidate", "release candidate", "rc", "candidate-release", "candidate release", "cr"],
            SemanticVersionType::Snapshot => vec!["snapshot", "snap"],
            SemanticVersionType::Release => vec!["release"],
        }
    }
}

impl Default for SemanticVersionType {
    fn default() -> Self {
        Self::Release
    }
}