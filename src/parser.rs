use std::path::PathBuf;

use crate::cli::ManagersArgs;

pub struct ParsedDependencies {
    file_paths: PathBuf,
    output_path: PathBuf,
}

const DEPENDENCY_FILE_PATH: [(ManagersArgs, &str); 5] = [
    (ManagersArgs::NPM, "package-lock.json"),
    (ManagersArgs::YARN, "yarn.lock"),
    (ManagersArgs::PNPM, "pnpm-lock.yaml"),
    (ManagersArgs::IOS, "ios/Podfile.lock"),
    (ManagersArgs::ANDROID, "android/build.gradle")
];


pub(crate) fn file_exists_in_directory(file_path: &str, root_directory: PathBuf) -> bool {
    let file_path = root_directory.join(file_path);

    file_path.exists()
}

pub(crate) fn handle_dependencies_files(manager: ManagersArgs, root_directory: PathBuf) -> Vec<PathBuf> {
    let mut lockfiles_paths: Vec<PathBuf> = vec![];

    match manager {
        ManagersArgs::NPM => {
            match file_exists_in_directory(DEPENDENCY_FILE_PATH[0].1, root_directory.clone()) {
                true => lockfiles_paths.push(DEPENDENCY_FILE_PATH[0].1.parse().unwrap()),
                _ => println!("{:?} not found", DEPENDENCY_FILE_PATH[0].1)
            };
        }
        ManagersArgs::YARN => {
            match file_exists_in_directory(DEPENDENCY_FILE_PATH[1].1, root_directory.clone()) {
                true => lockfiles_paths.push(DEPENDENCY_FILE_PATH[1].1.parse().unwrap()),
                _ => println!("{:?} not found", DEPENDENCY_FILE_PATH[1].1)
            };
        }
        ManagersArgs::PNPM => {
            match file_exists_in_directory(DEPENDENCY_FILE_PATH[2].1, root_directory.clone()) {
                true => lockfiles_paths.push(DEPENDENCY_FILE_PATH[2].1.parse().unwrap()),
                _ => println!("{:?} not found", DEPENDENCY_FILE_PATH[2].1)
            };
        }
        ManagersArgs::IOS => {
            match file_exists_in_directory(DEPENDENCY_FILE_PATH[3].1, root_directory.clone()) {
                true => lockfiles_paths.push(DEPENDENCY_FILE_PATH[3].1.parse().unwrap()),
                _ => println!("{:?} not found", DEPENDENCY_FILE_PATH[3].1)
            };
        }
        ManagersArgs::ANDROID => {
            match file_exists_in_directory(DEPENDENCY_FILE_PATH[4].1, root_directory.clone()) {
                true => lockfiles_paths.push(DEPENDENCY_FILE_PATH[4].1.parse().unwrap()),
                _ => println!("{:?} not found", DEPENDENCY_FILE_PATH[4].1)
            };
        }
        ManagersArgs::All => {
            for (.., filepath) in DEPENDENCY_FILE_PATH {
                match file_exists_in_directory(filepath, root_directory.clone()) {
                    true => lockfiles_paths.push(filepath.parse().unwrap()),
                    _ => println!("{:?} not found", filepath)
                };
            }
        }
    }

    lockfiles_paths
}

fn parse_package_lock(file_path: &str) {}

fn parse_yarn_lock(file_path: &str) {}

fn parse_pnpm_lock(file_path: &str) {}

fn parse_podlock(file_path: &str) {}

fn parse_settings_graddle(file_path: &str) {}
