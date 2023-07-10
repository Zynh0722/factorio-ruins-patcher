mod args;
mod config;
mod patch;
mod util;

use std::{fs::File, io::Read};

use clap::Parser;

use crate::{
    args::Args,
    config::{generate_config_dir, get_config_path},
    patch::fetch_compiled_patches,
    util::recursive_file_list,
};

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

    let (patch, _config_type) = fetch_compiled_patches();

    let target = args.target.or_else(|| std::env::current_dir().ok());
    if !target.is_some() {
        println!("Unable to access target directory");
        return;
    }
    // Safety: Look above...
    let target = unsafe { target.unwrap_unchecked() };
    let target_files = recursive_file_list(&target);

    println!("Files to be patched: {}", target_files.len());

    for path in target_files {
        let mut buf = String::new();
        File::open(&path).unwrap().read_to_string(&mut buf).unwrap();

        patch(&buf);
    }
}
