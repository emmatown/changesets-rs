use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemverType {
    Major,
    Minor,
    Patch,
}

impl SemverType {
    fn as_str(&self) -> &'static str {
        match self {
            SemverType::Major => "major",
            SemverType::Minor => "minor",
            SemverType::Patch => "patch",
        }
    }
}

impl fmt::Display for SemverType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
