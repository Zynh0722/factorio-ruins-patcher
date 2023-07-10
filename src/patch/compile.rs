use regex::Regex;

use super::{fetch_enabled_patches, Action, ConfigType, Patch};

type CompiledPatch = dyn Fn(&str) -> String;

pub fn replace_function(match_pattern: Regex, replace_pattern: Regex) -> Box<CompiledPatch> {
    Box::new(move |str| {
        let str = str.to_owned();

        let out = match_pattern.replace_all(&str, replace_pattern.as_str());

        out.to_string()
    })
}

// TODO: Make this actually do stuff lmao
pub fn delete_function(_match_pattern: Regex) -> Box<CompiledPatch> {
    Box::new(move |str| str.to_owned())
}

pub fn compile_patches(patches: Vec<Box<CompiledPatch>>) -> Box<CompiledPatch> {
    Box::new(move |str| {
        let mut out = str.to_string();

        patches.iter().for_each(|patch| {
            out = patch(&out);
        });

        out
    })
}

pub fn get_compiled_patches(patches: Vec<Patch>) -> Box<CompiledPatch> {
    compile_patches(
        patches
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
            .map(|(match_pattern, action)| match action {
                Some(replace_pattern) => replace_function(match_pattern, replace_pattern),
                None => delete_function(match_pattern),
            })
            .collect(),
    )
}

pub fn fetch_compiled_patches() -> (Box<CompiledPatch>, ConfigType) {
    let (patches, config_type) = fetch_enabled_patches();

    (get_compiled_patches(patches), config_type)
}
