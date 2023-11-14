// Remove . from file path
#[macro_export]
macro_rules! format_file_path {
    ($path:expr) => {{
        use std::path::{Component, PathBuf};

        $path.components()
            .fold(PathBuf::new(), |mut acc, comp| match comp {
                Component::ParentDir => { acc.pop(); acc }
                Component::CurDir => acc,
                _ => { acc.push(comp); acc }
            })
    }};
}

#[macro_export]
macro_rules! check_and_push_lockfile {
    ($file:expr, $root_directory:expr, $lockfile_paths:expr) => {
        if file_exists_in_directory($file, $root_directory.clone()) {
            $lockfile_paths.push($file.parse().unwrap());
        }
    };
}
