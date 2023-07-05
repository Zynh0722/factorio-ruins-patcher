use std::io::BufReader;

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

    let file = std::fs::File::open("./patches/convert_poles.json").unwrap();
    let buf_file = BufReader::new(file);

    let patch_file: PatchFile = serde_json::from_reader(buf_file).unwrap();

    println!("{args:?}");
    println!("{patch_file:?}");
}
