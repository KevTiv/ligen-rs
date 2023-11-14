use std::path::{Path, PathBuf};

use crate::cli::ManagersArgs;

pub struct ParsedDependencies {
    file_paths: PathBuf,
    output_path: PathBuf,
}

const NODE_DEPENDENCY_FILE_PATH: [(ManagersArgs, &str); 3] = [
    (ManagersArgs::NPM, "package-lock.json"),
    (ManagersArgs::YARN, "yarn.lock"),
    (ManagersArgs::PNPM, "pnpm-lock.yaml")
];

const NATIVE_DEPENDENCY_FILE_PATH: [(ManagersArgs, &str); 2] = [
    (ManagersArgs::IOS, "ios/Podfile.lock"),
    (ManagersArgs::ANDROID, "android/build.gradle")
];
struct DependencyFile {
    npm: String,
    yarn: String,
    pnpm: String,
    ios: String,
    android: String,
}

impl Default for DependencyFile {
    fn default() -> Self {
        DependencyFile {
            npm: "package-lock.json".to_string(),
            yarn: "yarn.lock".to_string(),
            pnpm: "pnpm-lock.yaml".to_string(),
            ios: "ios/Podfile.lock".to_string(),
            android: "android/build.gradle".to_string(),
        }
    }
}

struct HandleFile {
    file_exists_in_directory: (String, PathBuf),
    update_lock_file_path: (Vec<PathBuf>, PathBuf, PathBuf)
}

impl HandleFile for HandleFilesPath {
    fn file_exists_in_directory(file_path: &str, root_directory: PathBuf) -> bool {
        let file_path = root_directory.join(file_path);
        file_path.exists()
    }
    fn update_lock_file_path(mut lockfilepaths: Vec<PathBuf>, lockfilepath: &str, root_directory: PathBuf){
        Ok(if file_exists_in_directory(lockfilepath, root_directory.clone()) {
            lockfilepaths.push(lockfilepath.parse().unwrap())
        }).expect("Error occurred")
    }
}


pub(crate) fn file_exists_in_directory(file_path: &str, root_directory: PathBuf) -> bool {
    let file_path = root_directory.join(file_path);

    file_path.exists()
}

pub(crate) fn handle_dependencies_files(manager: ManagersArgs, root_directory: PathBuf) -> Vec<PathBuf> {
    let mut lockfilepaths: Vec<PathBuf> = vec![];

    match manager {
        ManagersArgs::NPM => {
            HandleFile::updateLockFilePath(
                lockfilepaths,
                NODE_DEPENDENCY_FILE_PATH[0].1,
                root_directory.clone()
            )
        }
        ManagersArgs::YARN => {
            HandleFilesPath::updateLockFilePath(
                lockfilepaths,
                NODE_DEPENDENCY_FILE_PATH[1].1,
                root_directory.clone()
            )
        }
        ManagersArgs::PNPM => {
            HandleFilesPath::updateLockFilePath(
                lockfilepaths,
                NODE_DEPENDENCY_FILE_PATH[2].1,
                root_directory.clone()
            )
        }
        ManagersArgs::IOS => {
            HandleFilesPath::updateLockFilePath(
                lockfilepaths,
                NODE_DEPENDENCY_FILE_PATH[3].1,
                root_directory.clone()
            )
        }
        ManagersArgs::ANDROID => {
            HandleFilesPath::updateLockFilePath(
                lockfilepaths,
                NODE_DEPENDENCY_FILE_PATH[4].1,
                root_directory.clone()
            )
        }
    }

    Ok::<Vec<PathBuf>, String>(lockfilepaths)
        .expect("Error: Something went wrong while locating dependencies files.")
}

fn parse_package_lock(file_path: &str) {}

fn parse_yarn_lock(file_path: &str) {}

fn parse_pnpm_lock(file_path: &str) {}

fn parse_podlock(file_path: &str) {}

fn parse_settings_graddle(file_path: &str) {}
