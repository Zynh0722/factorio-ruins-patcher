use std::{env, fs, io::Write, path::Path};

use serde_json::Value;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("default_patches.rs");
    let mut patches_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest_path)
        .unwrap();

    let patches_dir = fs::read_dir("patches").unwrap();

    for file in patches_dir {
        let file_name = &file.unwrap().file_name();
        let variable_name = file_name.to_string_lossy();
        let variable_name = variable_name.split(".").next().unwrap().to_uppercase();
        let file_path = Path::new("patches").join(&file_name);

        let file_contents: Value =
            serde_json::from_reader(fs::File::open(file_path).unwrap()).unwrap();

        writeln!(
            patches_file,
            "pub const {variable_name}: &str = r#\"{file_contents}\"#;"
        )
        .unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=patches");
}
