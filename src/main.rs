use std::io::BufReader;

mod args;
mod config;
mod patch;

use clap::Parser;

use crate::{
    args::Args,
    config::{generate_config_dir, get_config_path},
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

    let file = std::fs::File::open("./patches/convert_poles.json").unwrap();
    let buf_file = BufReader::new(file);

    let patch_file: PatchFile = serde_json::from_reader(buf_file).unwrap();

    println!("{args:?}");
    println!("{patch_file:?}");
}
