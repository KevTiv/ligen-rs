use crate::cli::{parse_cli_args, ParsedArgs};
use crate::parser::handle_dependencies_files;

mod cli;
mod parser;
mod write;
mod macros;

fn main() {
    let ParsedArgs { manager, root, output } = parse_cli_args();
    let dependency_lockfiles = handle_dependencies_files(manager, root);

    println!("file found: {:?}. Writing to {:?}", dependency_lockfiles, output)
}
