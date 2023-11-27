use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
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
    packages: Option<HashMap<String, PackageLockDevDependencies>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageLockDevDependencies {
    name: Option<String>,
    version: Option<String>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PnpmLockFile {
    dependencies: HashMap<String, Option<PnpmLockSpecifier>>,
    dev_dependencies: HashMap<String, Option<PnpmLockSpecifier>>,
    packages: HashMap<String, Option<PnpmLockSpecifier>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PnpmLockSpecifier {
    specifier: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageJson {
    name: Option<String>,
    description: Option<String>,
    repository: Option<Repository>,
    author: Option<String>,
    license: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Repository {
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedPackageJson {
    pub(crate) name: String,
    description: String,
    repository_url: String,
    author: String,
    license: String,
    license_url: String,
}

struct DependencyFile;

trait Parser {
    fn parse_package_lock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    fn parse_yarn_lock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn parse_pnpm_lock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn parse_podlock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    fn parse_settings_graddle(lockfile_path: &str) -> io::Result<()>;
}

trait FileParser {
    fn get_node_module_package_info(
        node_modules_path: Vec<String>,
        root_directory: &PathBuf,
    ) -> Vec<ParsedPackageJson>;
    fn read_file(file_path: &str) -> io::Result<String>;
    fn file_exists_in_directory(file_path: &str, root_directory: &PathBuf) -> bool;
    fn update_lock_file_path(
        lockfilepaths: &mut Vec<PathBuf>,
        lockfilepath: &str,
        root_directory: &PathBuf,
    );
    fn extract_yaml_library_name(key: &str) -> String;
    fn parse_podlock_dependency_line(line: &str) -> Option<(String, String)>;
}

fn get_license_file_url(node_module_path: &String, root_directory: &PathBuf) -> Option<String> {
    let mut license_url: String = "".to_string();
    let potential_license_file_paths = [
        "LICENSE",
        "license",
        "license.md",
        "LICENSE.md",
        "license.txt",
        "LICENSE.txt",
    ];

    potential_license_file_paths.iter().any(|&file_name| {
        let file_path = format!("{node_module_path}/{file_name}");
        if file_exists_in_directory(&file_path, root_directory) {
            license_url = format!("{node_module_path}/{file_name}");
            return true;
        }
        false
    });

    Some(license_url)
}

pub(crate) fn file_exists_in_directory(file_path: &str, root_directory: &PathBuf) -> bool {
    let file_path = root_directory.join(file_path);

    file_path.exists()
}

impl FileParser for DependencyFile {
    fn get_node_module_package_info(
        node_modules_path: Vec<String>,
        root_directory: &PathBuf,
    ) -> Vec<ParsedPackageJson> {
        let mut node_module_info: Vec<ParsedPackageJson> = Vec::new();

        for node_module_path in node_modules_path {
            let module_path = root_directory.join(&node_module_path);
            let package_json_path = module_path.join("package.json");

            if let Ok(node_package_json) =
                DependencyFile::read_file(package_json_path.to_str().unwrap_or_default())
            {
                match serde_json::from_str::<PackageJson>(&node_package_json) {
                    Ok(package_json) => {
                        let license_file_url =
                            get_license_file_url(&node_module_path, root_directory);

                        node_module_info.push(ParsedPackageJson {
                            name: package_json.name.unwrap_or("".to_string()),
                            description: package_json.description.unwrap_or("".to_string()),
                            repository_url: package_json
                                .repository
                                .unwrap_or(Repository {
                                    url: Some("".to_string()),
                                })
                                .url
                                .unwrap_or("".to_string()),
                            author: package_json.author.unwrap_or("".to_string()),
                            license: package_json.license.unwrap_or("".to_string()),
                            license_url: license_file_url.unwrap_or("".to_string()),
                        })
                    }
                    Err(error) => eprintln!("Error parsing package.json: {error}"),
                }
            } else {
                eprintln!("Failed to read file at {:?}", package_json_path);
            }
        }
        node_module_info
    }

    fn read_file(file_path: &str) -> io::Result<String> {
        let file = File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn file_exists_in_directory(file_path: &str, root_directory: &PathBuf) -> bool {
        let file_path = root_directory.clone().join(file_path);
        file_path.exists()
    }

    fn update_lock_file_path(
        lockfilepaths: &mut Vec<PathBuf>,
        lockfilepath: &str,
        root_directory: &PathBuf,
    ) {
        if Self::file_exists_in_directory(lockfilepath, &root_directory) {
            lockfilepaths.push(root_directory.join(lockfilepath));
        }
    }

    fn extract_yaml_library_name(key: &str) -> String {
        // Split the key by '/'
        let parts: Vec<&str> = key.split('/').collect();

        // The package name is usually the last part for non-scoped packages,
        // or the last two parts for scoped packages.
        match parts.as_slice() {
            // Scoped package (e.g., "/@babel/code-frame/7.10.4")
            [_, scope, rest, ..] if scope.starts_with('@') => {
                let name = rest.split('@').next().unwrap_or("");
                format!("{}/{}", scope, name)
            }
            // Non-scoped package (e.g., "/lodash/4.17.15")
            [_, rest, ..] => {
                let name = rest.split('@').next().unwrap_or("");
                name.to_string()
            }
            // Other cases (e.g., malformed or unexpected format)
            _ => String::new(),
        }
    }

    fn parse_podlock_dependency_line(line: &str) -> Option<(String, String)> {
        let line = line.trim();
        if line.is_empty() || line.chars().all(char::is_uppercase) {
            return None;
        }

        if let Some((lib_name, _)) = line.split_once('(') {
            let lib_name = lib_name.trim().trim_matches('-').to_string();

            let path = if line.contains("~>") {
                "./Pods".to_string()
            } else {
                line.split(" (from `")
                    .nth(1)
                    .unwrap_or("")
                    .trim_end_matches(')')
                    .trim()
                    .to_string()
            };

            return Some((lib_name, path));
        }

        None
    }
}

impl Parser for DependencyFile {
    fn parse_package_lock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        /// Parses a package-lock.json file and returns a list of dependencies.
        ///
        /// # Parameters
        ///
        /// * `lockfile_path` - The path to the package-lock.json file.
        ///
        /// # Returns
        ///
        /// A `Vec` of dependencies, or an error if the file could not be parsed.
        let file_content = DependencyFile::read_file(lockfile_path).unwrap();
        let package_lock_json = serde_json::from_str::<PackageLockJson>(&file_content);

        match package_lock_json {
            Ok(package_lock_json) => {
                let mut dependencies = Vec::new();
                if let Some(packages) = package_lock_json.packages {
                    for (package_name, _) in packages {
                        dependencies.push(package_name);
                    }
                }
                Ok(dependencies)
            }
            Err(error) => Err(Box::new(error)),
        }
    }

    fn parse_yarn_lock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        /// Parses a yarn.lock file and returns a list of dependencies.
        ///
        /// # Parameters
        ///
        /// * `lockfile_path` - The path to the yarn.lock file.
        ///
        /// # Returns
        ///
        /// A `Vec` of dependencies, or an error if the file could not be parsed.
        let yarn_lock = <DependencyFile as FileParser>::read_file(lockfile_path);

        match yarn_lock {
            Ok(yarn_lock) => {
                let mut dependencies = Vec::new();
                let mut seen = HashSet::new();

                for line in yarn_lock.lines() {
                    if let Some(colon_index) = line.find(':') {
                        let package_identifiers = line[..colon_index].split(',');

                        for package_identifier in package_identifiers {
                            let trimmed_identifier = package_identifier.trim();

                            let library_name = if trimmed_identifier.starts_with('@') {
                                // Scoped package
                                if let Some(slash_index) = trimmed_identifier.find('/') {
                                    &trimmed_identifier[..trimmed_identifier[slash_index..]
                                        .find('@')
                                        .map_or(trimmed_identifier.len(), |idx_to_trim| {
                                            slash_index + idx_to_trim
                                        })]
                                } else {
                                    continue; // Invalid format, skip
                                }
                            } else {
                                // Regular package
                                &trimmed_identifier[..trimmed_identifier
                                    .find('@')
                                    .unwrap_or(trimmed_identifier.len())]
                            };

                            if library_name.to_string() != ""
                                && seen.insert(library_name.to_string())
                            {
                                dependencies.push(format!("node_modules/{library_name}"));
                            }
                        }
                    }
                }
                Ok(dependencies)
            }
            Err(error) => Err(Box::new(error)),
        }
    }

    fn parse_pnpm_lock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        /// Parses a pnpm-lock.yaml file and returns a list of dependencies.
        ///
        /// # Parameters
        ///
        /// * `lockfile_path` - The path to the pnpm-lock.yaml file.
        ///
        /// # Returns
        ///
        /// A `Vec` of dependencies, or an error if the file could not be parsed.
        let pnpm_lock = <DependencyFile as FileParser>::read_file(lockfile_path);
        match pnpm_lock {
            Ok(pnpm_lock) => {
                let pnpm_lock: PnpmLockFile = serde_yaml::from_str(&pnpm_lock)?;
                let mut dependencies = Vec::new();
                let mut seen = std::collections::HashSet::new();

                for package in pnpm_lock
                    .dependencies
                    .keys()
                    .chain(pnpm_lock.dev_dependencies.keys())
                    .chain(pnpm_lock.packages.keys())
                {
                    match <DependencyFile as FileParser>::extract_yaml_library_name(package) {
                        formatted_name => {
                            if formatted_name.len() != 0 && seen.insert(formatted_name.clone()) {
                                dependencies.push(format!("node_modules/{}", formatted_name));
                            }
                        }
                    }
                }
                Ok(dependencies)
            }
            Err(error) => Err(Box::new(error)),
        }
    }

    fn parse_podlock(lockfile_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let podlock_file = <DependencyFile as FileParser>::read_file(lockfile_path);
        let mut dependencies = Vec::new();
        let mut parsed_dependencies = Vec::new();
        match podlock_file {
            Ok(podlock_file) => {
                let mut is_dependencies_section = false;
                for line in podlock_file.lines() {
                    if line.trim() == "DEPENDENCIES:" {
                        is_dependencies_section = true;
                        continue;
                    }

                    if is_dependencies_section {
                        if line.chars().all(|c| c.is_uppercase() || c.is_whitespace())
                            && !line.trim().is_empty()
                        {
                            break; // Break if we encounter a fully capitalized word (new section)
                        }

                        if let Some((lib_name, path)) =
                            <DependencyFile as FileParser>::parse_podlock_dependency_line(&line)
                        {
                            dependencies.push((lib_name, path));
                        }
                    }
                }
                println!("{:?}", dependencies)
            }
            _ => (),
        }
        return Ok(parsed_dependencies);
    }

    fn parse_settings_graddle(lockfile_path: &str) -> io::Result<()> {
        let content = <DependencyFile as FileParser>::read_file(lockfile_path);
        Result::from(Ok(println!("Here 5 parse_package_lock: {:?} ", content)))
    }
}

pub(crate) fn handle_dependencies_files(
    manager: ManagersArgs,
    root_directory: &PathBuf,
) -> Vec<PathBuf> {
    let mut lockfilepaths: Vec<PathBuf> = Vec::new();

    let file_path = match manager {
        ManagersArgs::NPM => "./package-lock.json".to_string(),
        ManagersArgs::YARN => "./yarn.lock".to_string(),
        ManagersArgs::PNPM => "./pnpm-lock.yaml".to_string(),
        ManagersArgs::IOS => "./ios/Podfile.lock".to_string(),
        ManagersArgs::ANDROID => "./android/build.gradle".to_string(),
    };

    if DependencyFile::file_exists_in_directory(&file_path, root_directory) {
        lockfilepaths.push(PathBuf::from(format_file_path!(
            root_directory.join(file_path)
        )));
    }

    lockfilepaths
}

pub(crate) fn parse_lock_file(
    manager: ManagersArgs,
    root: &PathBuf,
    lockfile_path: &str,
) -> Vec<ParsedPackageJson> {
    let mut parsed_dependencies: Vec<ParsedPackageJson> = Vec::new();
    let _ = match manager {
        ManagersArgs::NPM => {
            let dependencies = DependencyFile::parse_package_lock(lockfile_path);
            match dependencies {
                Ok(package_lock) => {
                    let found = DependencyFile::get_node_module_package_info(package_lock, &root);
                    parsed_dependencies = found;
                }
                Err(error) => eprintln!("{error}"),
            }
        }
        ManagersArgs::YARN => {
            let dependencies = DependencyFile::parse_yarn_lock(lockfile_path);
            match dependencies {
                Ok(package_lock) => {
                    let found = DependencyFile::get_node_module_package_info(package_lock, &root);
                    parsed_dependencies = found;
                }
                Err(error) => eprintln!("{error}"),
            }
        }
        ManagersArgs::PNPM => {
            let dependencies = DependencyFile::parse_pnpm_lock(lockfile_path);
            match dependencies {
                Ok(package_lock) => {
                    let found = DependencyFile::get_node_module_package_info(package_lock, &root);
                    parsed_dependencies = found;
                }
                Err(error) => eprintln!("{error}"),
            }
        }
        ManagersArgs::IOS => {
            let podlock_file = DependencyFile::parse_settings_graddle(lockfile_path);

            let dependencies = DependencyFile::parse_podlock(lockfile_path);
            ()
        }
        ManagersArgs::ANDROID => {
            let _ = DependencyFile::parse_settings_graddle(lockfile_path);
            ()
        }
    };
    parsed_dependencies
}
