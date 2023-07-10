use std::path::PathBuf;

pub fn recursive_file_list(dir: &PathBuf) -> Vec<PathBuf> {
    if dir.is_dir() {
        dir.read_dir()
            .unwrap()
            .flat_map(|entry| entry.map(|e| recursive_file_list(&e.path()))) // flatten to remove results
            .flatten() // flatten sub arrays
            .flat_map(|path| path.canonicalize()) // flatten results
            .collect()
    } else {
        vec![dir.to_owned()]
    }
}
