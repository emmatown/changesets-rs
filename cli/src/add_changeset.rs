use super::cargo_workspace_info::CargoWorkspaceInfo;
use changesets_parse::{Changeset, Release, SemverType};
use dialoguer::{Input, MultiSelect};
use enquirer::ColoredTheme;
use eyre::Result;
use std::collections::HashSet;
use std::fs::{create_dir, write};

pub fn add_changeset(cargo_workspace_info: CargoWorkspaceInfo) -> Result<()> {
    let changeset = create_changeset(cargo_workspace_info)?;
    println!("{:?}", changeset);
    let mut filename = ".changeset".to_string();
    filename.push_str(&human_id::id("-", false)[..]);
    filename.push_str(".md");
    create_dir(".changeset-rs")?;
    write::<_, String>(filename, changeset.into())?;

    Ok(())
}

fn prompt(packages: &Vec<&String>, prompt: &str) -> Result<HashSet<String>> {
    let selections = MultiSelect::with_theme(&ColoredTheme::default())
        .with_prompt(prompt)
        .items(&packages[..])
        .interact()?;
    let x = selections
        .into_iter()
        .map(|x| packages[x].clone())
        .collect();
    return Ok(x);
}

fn create_changeset(cargo_workspace_info: CargoWorkspaceInfo) -> Result<Changeset> {
    let mut releases = vec![];

    let items: Vec<&String> = cargo_workspace_info.members.keys().collect();
    let selections = prompt(
        &items,
        "What packages would you like to create a changeset for?",
    )?;

    let majors = prompt(&items, "What packages should have a major bump?")?;

    let mut packages_for_minor_or_patch = selections.clone();

    for pkg in majors.iter() {
        releases.push(Release {
            kind: SemverType::Major,
            package: pkg.clone(),
        });
        packages_for_minor_or_patch.remove(pkg);
    }

    if packages_for_minor_or_patch.len() != 0 {
        let minors: HashSet<String> = prompt(
            &packages_for_minor_or_patch.iter().collect(),
            "What packages should have a minor bump?",
        )?
        .iter()
        .map(|str| str.clone())
        .collect();

        for pkg in packages_for_minor_or_patch.iter() {
            if minors.contains(pkg) {
                releases.push(Release {
                    kind: SemverType::Minor,
                    package: pkg.clone(),
                })
            } else {
                releases.push(Release {
                    kind: SemverType::Patch,
                    package: pkg.clone(),
                })
            }
        }
    }

    let summary = Input::new()
        .with_prompt("What is the summary of your changes?")
        .interact()?;

    Ok(Changeset { summary, releases })
}
