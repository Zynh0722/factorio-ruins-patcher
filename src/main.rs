use std::{
    io::{self, BufReader},
    path::PathBuf,
};

use std::io::Write;

mod args;
mod patch;

use clap::Parser;

use crate::{args::Args, patch::PatchFile};

include!(concat!(env!("OUT_DIR"), "/default_patches.rs"));

fn main() {
    let mut args = Args::parse();
    if args.dry_run {
        args.verbose = true;
    }

    if args.get_config_path {
        print!("{:?}", get_config_path());
        return;
    }

    if args.generate_config {
        generate_config_dir().expect("Unable to generate config dir");
        return;
    }

    let file = std::fs::File::open("./patches/convert_poles.json").unwrap();
    let buf_file = BufReader::new(file);

    let patch_file: PatchFile = serde_json::from_reader(buf_file).unwrap();

    println!("{args:?}");
    println!("{patch_file:?}");
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .expect("Unsupported Platform")
        .join("factorio-ruins-patcher")
}

fn generate_config_dir() -> io::Result<()> {
    let patches_path = get_config_path().join("patches");

    std::fs::create_dir_all(&patches_path)?;

    for (key, value) in PATCHES.entries() {
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
