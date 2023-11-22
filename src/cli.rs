use std::env;
use std::path::PathBuf;

use clap::{arg, Command, Parser, ValueEnum};

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
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    #[clap(value_enum)]
    manager: ManagersArgs,
    #[arg(short, long)]
    #[clap(value_parser)]
    path: PathBuf,
    #[arg(short, long, default_value = "./dependencies.json")]
    #[clap(value_parser)]
    output: Option<PathBuf>,
}

pub struct ParsedArgs {
    pub manager: ManagersArgs,
    pub root: PathBuf,
    pub output: PathBuf,
}

// pub fn cli() -> ParsedArgs {
//     let Cli {
//         manager,
//         path,
//         output,
//     } = Cli::parse();
//
//     let cwd = env::current_dir().unwrap();
//     let working_directory = format_file_path!(cwd.join(path));
//     let output_path = format_file_path!(match output {
//         None => working_directory.join("dependencies-licenses.json"),
//         _ => working_directory.join(output.unwrap()),
//     });
//
//     ParsedArgs {
//         manager,
//         root: working_directory,
//         output: output_path,
//     }
// }

pub(crate) fn cli() -> ParsedArgs {
    let ligen = Command::new("ligen")
        .about("List your RN project dependencies licenses")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("manager")
                .about("List dependencies manger. NPM | YARN | PNPM | cocoapods | android")
                .arg(arg!(<MANAGER> "Manager"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("Input Path")
                .about("Path to project")
                .arg(arg!(<PATH> "Path"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("Output Path")
                .about("path to dependencies to project")
                .arg(arg!(<OUTPUT> "Output Path"))
                .arg_required_else_help(true),
        );

    let Cli {
        manager,
        path,
        output,
    } = Cli::parse();

    let cwd = env::current_dir().unwrap();
    let working_directory = format_file_path!(cwd.join(path));
    let output_path = format_file_path!(match output {
        None => working_directory.join("dependencies-licenses.json"),
        _ => working_directory.join(output.unwrap()),
    });

    ParsedArgs {
        manager,
        root: working_directory,
        output: output_path,
    }
}
