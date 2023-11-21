use serde_json::{Map, Value};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

pub(crate) fn write_node_dependencies_to_file(
    vec: Map<String, Value>,
    file_path: &str,
) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, &vec)?;
    writer.flush()?;
    Ok(())
}
