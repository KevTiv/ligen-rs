use crate::cli::{cli, ParsedArgs};
use crate::parser::{handle_dependencies_files, parse_lock_file, ParsedPackageJson};
use crate::write::write_vec_to_file;
use serde_json::Value;
use std::ops::Deref;

mod cli;
mod macros;
mod parser;
mod write;

fn main() {
    let ParsedArgs {
        manager,
        root,
        output,
    } = cli();

    let dependency_lockfiles = handle_dependencies_files(manager, &root);

    for lockfile in dependency_lockfiles.iter() {
        match lockfile.to_str() {
            Some(path) => {
                let output_path = match output.to_str().unwrap().len() > 0 {
                    true => output.to_str(),
                    _ => Option::from("./dependencies.json"),
                };

                let parsed_dependencies = parse_lock_file(manager, &root, path);
                let parsed_dependencies_json: serde_json::Map<String, Value> = parsed_dependencies
                    .iter()
                    .map(|pkg| {
                        let pkg_value =
                            serde_json::to_value(pkg).expect("Failed to serialize package");
                        (pkg.name.clone(), pkg_value)
                    })
                    .collect();

                let _ = write_vec_to_file(parsed_dependencies_json, output_path.unwrap());
                ()
            }
            _ => eprint!("Something went wrong"),
        }
    }
}
