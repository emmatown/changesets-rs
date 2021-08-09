use core::convert::TryFrom;
use eyre::{Report, Result, WrapErr};
use serde::{Deserialize, Serialize};
mod semver_type;

pub use semver_type::SemverType;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct PackageName(String);

#[derive(Debug, PartialEq)]
pub struct Release {
    pub kind: SemverType,
    pub package: String,
}

#[derive(Debug, PartialEq)]
pub struct Changeset {
    pub releases: Vec<Release>,
    pub summary: String,
}

fn parse_frontmatter(source: &str) -> Result<Vec<Release>> {
    let mut releases: Vec<Release> = vec![];
    let deserialized_map: Vec<(String, SemverType)> =
        serde_yaml::from_str(source).wrap_err("Error when parsing YAML in Changeset")?;
    for (package, kind) in deserialized_map.iter() {
        releases.push(Release {
            kind: kind.clone(),
            package: package.clone(),
        })
    }
    Ok(releases)
}

impl TryFrom<&str> for Changeset {
    type Error = Report;
    fn try_from(contents: &str) -> Result<Self, Self::Error> {
        // TODO: maybe don't use regex
        match find_frontmatter(contents) {
            Some((fm_start, fm_end, content_start)) => {
                let yaml_str = &contents[fm_start..fm_end];
                let message = &contents[content_start..];
                let releases = parse_frontmatter(yaml_str)?;
                Ok(Changeset {
                    summary: message.trim().to_string(),
                    releases,
                })
            }
            None => Err(Report::msg("Changeset did not match pattern")),
        }
    }
}

impl Into<String> for Changeset {
    fn into(self) -> String {
        let mut stringified: String = "---\n".into();
        for release in self.releases {
            let formatted = format!("\"{}\": {}\n", release.package, release.kind);
            stringified += formatted.as_str();
        }
        format!("{}---\n\n{}\n", stringified, self.summary.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_case() {
        let case = r"---
'cool-package': minor
---
        Nice simple summary
";
        let changeset = Changeset::try_from(case).unwrap();
        assert_eq!(
            changeset,
            Changeset {
                summary: "Nice simple summary".to_string(),
                releases: vec![Release {
                    kind: SemverType::Minor,
                    package: "cool-package".to_string()
                }]
            }
        )
    }
    #[test]
    fn major_minor_patch() {
        let case = r"---
'cool-package': minor
'cool-package2': major
'cool-package3': patch
---
Nice simple summary
";
        let changeset = Changeset::try_from(case).unwrap();
        assert_eq!(
            changeset,
            Changeset {
                summary: "Nice simple summary".to_owned(),
                releases: vec![
                    Release {
                        kind: SemverType::Minor,
                        package: "cool-package".to_string()
                    },
                    Release {
                        kind: SemverType::Major,
                        package: "cool-package2".to_string()
                    },
                    Release {
                        kind: SemverType::Patch,
                        package: "cool-package3".to_string()
                    }
                ]
            }
        )
    }
}

// https://github.com/azdle/rust-frontmatter/blob/master/src/lib.rs#L9-L37
fn find_frontmatter(contents: &str) -> Option<(usize, usize, usize)> {
    match contents.starts_with("---\n") {
        true => {
            let slice_after_marker = &contents[4..];
            let fm_end = slice_after_marker.find("---\n");
            fm_end.and_then(|fm_end| Some((4, fm_end + 4, fm_end + 2 * 4)))
        }
        false => None,
    }
}
