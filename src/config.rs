use std::{io, path::PathBuf};

use std::io::Write;

#[allow(unused)]
pub fn config_exists() -> bool {
    get_config_path().is_dir()
}

#[allow(unused)]
pub fn patches_exists() -> bool {
    get_patches_path().is_dir()
}

pub fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .expect("Unsupported Platform")
        .join("factorio-ruins-patcher")
}

pub fn get_patches_path() -> PathBuf {
    get_config_path().join("patches")
}

pub fn generate_config_dir() -> io::Result<()> {
    let patches_path = get_config_path().join("patches");

    std::fs::create_dir_all(&patches_path)?;

    for (key, value) in crate::PATCHES.entries() {
        let file_path = patches_path.join(key.to_string() + ".json");
        let parsed_json: serde_json::Value = serde_json::from_str(value).unwrap();

        let mut file_handle = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)?;

        write!(file_handle, "{parsed_json:#}")?;
    }

    Ok(())
}
