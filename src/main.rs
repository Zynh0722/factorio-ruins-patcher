mod args;
mod config;
mod patch;

use clap::Parser;

use patch::fetch_enabled_patches;

use crate::{
    args::Args,
    config::{generate_config_dir, get_config_path},
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

    let (patches, _config_type) = fetch_enabled_patches();

    patches.iter().for_each(|patch| println!("{patch:#?}"));
}
