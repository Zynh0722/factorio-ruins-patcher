use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Action {
    /// A regex replace pattern. It has access to the capture groups from the Patch pattern
    /// https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace
    Replace { pattern: String },
    /// Deletes the entire line the Patch pattern was found on
    Delete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patch {
    /// A regex pattern used for locating a patch location, and capturing information
    pattern: String,
    action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatchFile {
    #[serde(default)]
    priority: i32,
    patches: Vec<Patch>,
}
