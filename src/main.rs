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
            Some(path) => parse_lock_file(manager, &root, path),
            _ => eprint!("Something went wrong"),
        };
    }
    // for lockfilePath in &dependency_lockfiles {
    //     let file_path = lockfilePath;
    //
    //     match file_path.to_str() {
    //         Some(path) => {
    //             parse_lock_file(manager, path)
    //         },
    //         None => {
    //             panic!("Error reading: {:?}", file_path)
    //         }
    //     }
    //     // println!("file : {:?}. ", file_path);
    //
    // }

    // println!("file found: {:?}. Writing to {:?}", dependency_lockfiles, output)
    ()
}
