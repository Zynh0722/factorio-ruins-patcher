use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Perform a non-destructive pass using all patches
    ///
    /// Verbosely log both the selected patches and their effects on
    /// the target directory without actually editing any files
    ///
    /// This uses the same logging as -v or --verbose
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    pub dry_run: bool,
    /// Enable verbose logging
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    pub verbose: bool,
    /// Specify the target directory to patch
    ///
    /// This directory will be crawled recursively, and it will
    /// patch any lua files found
    #[arg(short, long, verbatim_doc_comment)]
    pub target: Option<PathBuf>,
    /// Specify additonal config files or directories
    ///
    /// You can add any number of files, for examples look
    /// at the default patch configs.
    ///
    /// Directories are explores recursively for config files
    #[arg(short, long, verbatim_doc_comment)]
    pub config: Option<Vec<PathBuf>>,
    /// Generates the config directory
    ///
    /// This also resets the default patch files
    /// TODO: link to patches directory on github
    ///
    /// This uses `dirs::config_dir` under the hood
    /// https://docs.rs/dirs/latest/dirs/fn.config_dir.html#
    #[arg(long, verbatim_doc_comment)]
    pub generate_config: bool,
    /// Simply prints the location of the config path
    #[arg(long, verbatim_doc_comment)]
    pub get_config_path: bool,
}
