use std::fs::File;

use crate::config::{get_patches_path, patches_exists};

mod parser;

pub use parser::*;

include!(concat!(env!("OUT_DIR"), "/default_patches.rs"));

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
