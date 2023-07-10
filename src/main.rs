mod args;
mod config;
mod patch;
mod util;

use clap::Parser;

use patch::{fetch_enabled_patches, Action};
use regex::Regex;

use crate::{
    args::Args,
    config::{generate_config_dir, get_config_path},
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

    let (patches, _config_type) = fetch_enabled_patches();

    let patches: Vec<(Regex, Option<Regex>)> = patches
        .into_iter()
        .map(|patch| {
            (
                Regex::new(&patch.pattern).unwrap(),
                if let Action::Replace { pattern } = patch.action {
                    Some(Regex::new(&pattern).unwrap())
                } else {
                    None
                },
            )
        })
        .collect();

    let target = args.target.or_else(|| std::env::current_dir().ok());
    if !target.is_some() {
        println!("Unable to access target directory");
        return;
    }
    // Safety: Look above...
    let target = unsafe { target.unwrap_unchecked() };
    let _target_files = recursive_file_list(&target);

    patches.iter().for_each(|patch| println!("{patch:#?}"));
}
