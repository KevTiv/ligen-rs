use std::env;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::format_file_path;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ManagersArgs {
    NPM,
    YARN,
    PNPM,
    IOS,
    ANDROID,
    All,
}

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(value_enum)]
    manager: ManagersArgs,
    #[clap(value_parser)]
    path: PathBuf,
    #[clap(value_parser)]
    output: Option<PathBuf>,
}

pub struct ParsedArgs {
    pub manager: ManagersArgs,
    pub root: PathBuf,
    pub output: PathBuf,
}

pub fn parse_cli_args() -> ParsedArgs {
    let Cli { manager, path, output } = Cli::parse();
    let cwd = env::current_dir().unwrap();
    let working_directory = format_file_path!(cwd.join(path));
    let output_path = format_file_path!(match output {
        None => working_directory.join("dependencies-licenses.json"),
        _ => working_directory.join(output.unwrap()),
    });

    ParsedArgs { manager, root: working_directory, output: output_path }
}
