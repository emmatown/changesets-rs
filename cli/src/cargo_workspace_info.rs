use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cargo_metadata::{MetadataCommand, Package};

// based on https://github.com/aklitzke/dors/blob/master/src/lib.rs#L113-L141
pub struct CargoWorkspaceInfo {
    pub members: HashMap<String, Package>,
    pub root: PathBuf,
}

impl CargoWorkspaceInfo {
    pub fn new(dir: &Path) -> CargoWorkspaceInfo {
        let metadata = MetadataCommand::new().current_dir(&dir).exec().unwrap();
        let root = metadata.workspace_root;
        // allow O(1) referencing of package information
        let packages: HashMap<_, _> = metadata
            .packages
            .iter()
            .map(|package| (package.id.to_owned(), package))
            .collect();
        let members: HashMap<String, Package> = metadata
            .workspace_members
            .into_iter()
            .map(|member| {
                let package = packages[&member];
                (package.name.clone(), package.to_owned())
            })
            .collect();
        CargoWorkspaceInfo { members, root }
    }
}
