use crate::cli::{cli, ParsedArgs};
use crate::parser::{handle_dependencies_files, parse_lock_file};
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
                let parsed_dependencies = parse_lock_file(manager, &root, path);
            }
            _ => eprint!("Something went wrong"),
        };
    }
}
