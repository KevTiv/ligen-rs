use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};


use crate::cli::ManagersArgs;
use crate::format_file_path;

#[derive(Serialize, Deserialize, Debug)]
struct PackageLockJson {
    name: Option<String>,
    version: Option<String>,
    lockfile_version: Option<u32>,
    requires: Option<bool>,
    packages: Option<HashMap<String, Package>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: Option<String>,
    version: Option<String>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
}

struct DependencyFile;

trait Parser {
    fn parse_package_lock(lockfile_path: &str) -> Result<PackageLockJson, Box<dyn std::error::Error>>;
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
    fn parse_package_lock(lockfile_path: &str) -> Result<PackageLockJson, Box<dyn std::error::Error>> {
        let file_content = DependencyFile::read_file(lockfile_path).unwrap();

        match serde_json::from_str(&file_content) {
            Ok(package_lock_json) => {
                println!("Finished parsing! {:?}", package_lock_json);
                Ok(package_lock_json)
            },
            Err(e) => Err(Box::new(e))
        }
    }
    fn parse_yarn_lock(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here 2 parse_package_lock: {:?} ", lockfile_path)))
    }

    fn parse_pnpm_lock(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here 3 parse_package_lock: {:?} ", content)))
    }

    fn parse_podlock(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here 4 parse_package_lock: {:?} ", content)))
    }

    fn parse_settings_graddle(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here 5 parse_package_lock: {:?} ", content)))
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

pub(crate) fn parse_lock_file(manager: ManagersArgs, lockfile_path: &str) {
    let parsed_lock_file = match manager {
        ManagersArgs::NPM => {
            let package_lock = DependencyFile::parse_package_lock(lockfile_path);
            let mut file = File::create("output.txt");
            let _ = file.expect("Error with file").write_all(format!("{:?}", package_lock).as_ref());
            Ok(())
        },
        ManagersArgs::YARN => DependencyFile::parse_yarn_lock(lockfile_path),
        ManagersArgs::PNPM => DependencyFile::parse_pnpm_lock(lockfile_path),
        ManagersArgs::IOS => DependencyFile::parse_podlock(lockfile_path),
        ManagersArgs::ANDROID => DependencyFile::parse_settings_graddle(lockfile_path),
    };
}
