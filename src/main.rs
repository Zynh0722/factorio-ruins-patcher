use std::fs::File;

mod args;
mod config;
mod patch;

use clap::Parser;

use crate::{
    args::Args,
    config::{generate_config_dir, get_config_path, get_patches_path, patches_exists},
    patch::PatchFile,
};

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

    if patches_exists() {
        for patch in std::fs::read_dir(get_patches_path()).unwrap() {
            let patch: PatchFile =
                serde_json::from_reader(File::open(patch.unwrap().path()).unwrap()).unwrap();
            println!("{patch:?}");
        }
    } else {
        for patch in PATCHES.values() {
            let patch: PatchFile = serde_json::from_str(patch).unwrap();
            println!("{patch:?}");
        }
    }
}
