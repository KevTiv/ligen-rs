use std::io;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::PathBuf;
use serde_json::{json, Value};

use crate::cli::ManagersArgs;
use crate::format_file_path;

pub struct ParsedDependencies {
    file_paths: PathBuf,
    output_path: PathBuf,
}

struct DependencyFile;

trait Parser {
    fn parse_package_lock(lockfile_path: &str) -> io::Result<()>;
    fn parse_yarn_lock(lockfile_path: &str) -> io::Result<()>;

    fn parse_pnpm_lock(lockfile_path: &str) -> io::Result<()>;

    fn parse_podlock(lockfile_path: &str) -> io::Result<()>;

    fn parse_settings_graddle(lockfile_path: &str) -> io::Result<()>;
}

trait FileParser {
    fn read_file(file_path: &str) -> io::Result<String>;
    fn file_exists_in_directory(file_path: &str, root_directory: &PathBuf) -> bool;
    fn update_lock_file_path(lockfilepaths: &mut Vec<PathBuf>, lockfilepath: &str, root_directory: &PathBuf);
}

pub(crate) fn file_exists_in_directory(file_path: &str, root_directory: PathBuf) -> bool {
    let file_path = root_directory.join(file_path);

    file_path.exists()
}

impl FileParser for DependencyFile {
    fn read_file(file_path: &str) -> io::Result<String> {
        let file = File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(contents)
    }
    fn file_exists_in_directory(file_path: &str, root_directory: &PathBuf) -> bool {
        let file_path = root_directory.join(file_path);
        file_path.exists()
    }

    fn update_lock_file_path(lockfilepaths: &mut Vec<PathBuf>, lockfilepath: &str, root_directory: &PathBuf) {
        if Self::file_exists_in_directory(lockfilepath, &root_directory) {
            lockfilepaths.push(root_directory.join(lockfilepath));
        }
    }
}

impl Parser for DependencyFile {
    fn parse_package_lock(lockfile_path: &str) -> io::Result<()> {
        let raw_content = json!(<DependencyFile as FileParser>::read_file(lockfile_path));
        let content = (raw_content.ok());
        Ok(println!("content: {:?}", content))
        // buf_reader.read_to_string(&mut contents)?;
    }
    fn parse_yarn_lock(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here parse_package_lock: {:?} ", lockfile_path)))
    }

    fn parse_pnpm_lock(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here parse_package_lock: {:?} ", content)))
    }

    fn parse_podlock(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here parse_package_lock: {:?} ", content)))
    }

    fn parse_settings_graddle(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here parse_package_lock: {:?} ", content)))
    }
}

pub(crate) fn handle_dependencies_files(manager: ManagersArgs, root_directory: PathBuf) -> Vec<PathBuf> {
    let mut lockfilepaths: Vec<PathBuf> = vec![];

    let file_path = match manager {
        ManagersArgs::NPM => "./package-lock.json".to_string(),
        ManagersArgs::YARN => "./yarn.lock".to_string(),
        ManagersArgs::PNPM => "./pnpm-lock.yaml".to_string(),
        ManagersArgs::IOS => "./ios/Podfile.lock".to_string(),
        ManagersArgs::ANDROID => "./android/build.gradle".to_string(),
    };

    if DependencyFile::file_exists_in_directory(&file_path, &root_directory) {
        lockfilepaths.push(PathBuf::from(format_file_path!(root_directory.join(file_path))));
    }

    lockfilepaths
}

pub(crate) fn parse_lock_file(manager: ManagersArgs, lockfile_path: &str) -> io::Result<()> {
    match manager {
        ManagersArgs::NPM => {DependencyFile::parse_package_lock(lockfile_path)},
        ManagersArgs::YARN => DependencyFile::parse_yarn_lock(lockfile_path),
        ManagersArgs::PNPM => DependencyFile::parse_pnpm_lock(lockfile_path),
        ManagersArgs::IOS => DependencyFile::parse_podlock(lockfile_path),
        ManagersArgs::ANDROID => DependencyFile::parse_settings_graddle(lockfile_path),
    }
}
