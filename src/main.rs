mod args;
mod config;
mod patch;
mod util;

use core::fmt;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use clap::Parser;
use console::{style, Style};
use similar::{ChangeTag, TextDiff};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}
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

    for path in target_files {
        let mut buf = String::new();
        File::open(&path).unwrap().read_to_string(&mut buf).unwrap();

        let out = patch(&buf);

        if args.verbose {
            println!(
                "---------| {:-<1$}",
                path.file_name().unwrap().to_string_lossy().to_string() + " ",
                69 /* nice */
            );
            print_diff(&buf, &out);
        }

        if !args.dry_run && out != buf {
            fs::write(path, out).unwrap();
        }
    }
}

// This is copy pasted from the inline example on for similar
fn print_diff(a: &str, b: &str) {
    let diff = TextDiff::from_lines(a, b);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            println!("{:-^1$}", "-", 80);
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                print!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                );
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        print!("{}", s.apply_to(value).underlined().on_black());
                    } else {
                        print!("{}", s.apply_to(value));
                    }
                }
                if change.missing_newline() {
                    println!();
                }
            }
        }
    }
}
