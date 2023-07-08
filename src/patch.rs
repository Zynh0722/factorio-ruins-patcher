use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    config::{get_patches_path, patches_exists},
    PATCHES,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Action {
    /// A regex replace pattern. It has access to the capture groups from the Patch pattern
    /// https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace
    Replace { pattern: String },
    /// Deletes the entire line the Patch pattern was found on
    Delete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patch {
    /// A regex pattern used for locating a patch location, and capturing information
    pattern: String,
    action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchFile {
    #[serde(default)]
    priority: i32,
    #[serde(default = "default_enabled")]
    enabled: bool,
    patches: Vec<Patch>,
}

fn default_enabled() -> bool {
    true
}

pub enum ConfigType {
    Default,
    Config,
}

pub fn fetch_patches() -> (Vec<PatchFile>, ConfigType) {
    if patches_exists() {
        (
            std::fs::read_dir(get_patches_path())
                .unwrap()
                .flat_map(|dir_entry| dir_entry.map(|e| e.path()))
                .flat_map(|path| File::open(path))
                .map(|handle| serde_json::from_reader(handle).unwrap())
                .collect(),
            ConfigType::Config,
        )
    } else {
        (
            PATCHES
                .values()
                .flat_map(|str| serde_json::from_str(*str))
                .collect(),
            ConfigType::Default,
        )
    }
}
