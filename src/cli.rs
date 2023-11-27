use std::env;
use std::path::PathBuf;

use clap::{arg, Parser, ValueEnum};

use crate::format_file_path;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ManagersArgs {
    NPM,
    YARN,
    PNPM,
    IOS,
    ANDROID,
}

#[derive(Parser, Debug)]
#[command(author="Kevin Tivert", version="0.0.1", about="license generator lib", long_about = None)]
pub(crate) struct Cli {
    #[arg(short = 'm', long)]
    #[clap(value_enum)]
    manager: ManagersArgs,
    #[arg(short = 'p', long)]
    #[clap(value_parser)]
    path: PathBuf,
    #[arg(short, long, default_value = "./dependencies-licenses.json")]
    #[clap(value_parser)]
    output: Option<PathBuf>,
}

pub struct ParsedArgs {
    pub manager: ManagersArgs,
    pub root: PathBuf,
    pub output: PathBuf,
}

pub(crate) fn cli() -> ParsedArgs {
    let Cli {
        manager,
        path,
        output,
    } = Cli::parse();

    let cwd = env::current_dir().unwrap();
    let working_directory = format_file_path!(cwd.join(path));
    let output_path = format_file_path!(working_directory.join(output.unwrap()));

    ParsedArgs {
        manager,
        root: working_directory,
        output: output_path,
    }
}
