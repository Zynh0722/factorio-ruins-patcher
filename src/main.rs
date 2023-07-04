use std::{io::BufReader, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
    #[arg(short, long)]
    target: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Action {
    /// A regex replace pattern. It has access to the capture groups from the Patch pattern
    /// https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace
    Replace { pattern: String },
    /// Deletes the entire line the Patch pattern was found on
    Delete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Patch {
    /// A regex pattern used for locating a patch location, and capturing information
    pattern: String,
    action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PatchFile {
    #[serde(default)]
    priority: i32,
    patches: Vec<Patch>,
}

fn main() {
    let args = Args::parse();

    let file = std::fs::File::open("./patches/convert_poles.json").unwrap();
    let buf_file = BufReader::new(file);

    let patch_file: PatchFile = serde_json::from_reader(buf_file).unwrap();

    println!("{args:?}");
    println!("{patch_file:?}");
}
