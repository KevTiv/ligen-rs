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
