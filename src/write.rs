use crate::parser::ParsedPackageJson;
use serde_json::{Map, Value};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

fn write_node_dependencies_to_file() {}

fn write_ios_dependencies_to_file() {}

fn write_android_dependencies_to_file() {}

pub(crate) fn write_vec_to_file(vec: Map<String, Value>, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    println!("HERE ===> {:?}", file_path);

    for line in vec {
        writeln!(writer, "{:?}", line)?;
    }

    writer.flush()?;
    Ok(())
}
