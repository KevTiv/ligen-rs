use crate::parser::ParsedPackageJson;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

fn write_node_dependencies_to_file() {}

fn write_ios_dependencies_to_file() {}

fn write_android_dependencies_to_file() {}

pub(crate) fn write_vec_to_file(vec: Vec<ParsedPackageJson>, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    println!("HERE ===> {:?}", file_path);

    for line in vec {
        writeln!(writer, "{:?}", line)?;
    }

    writer.flush()?;
    Ok(())
}
