use std::path::{Path, PathBuf};

fn recurse_dir(v: &mut Vec<PathBuf>, dir: impl AsRef<Path>) {
    for entry in
        std::fs::read_dir(&dir).unwrap_or_else(|_| panic!("Unable to read dir: {:?}", dir.as_ref()))
    {
        let path = entry.expect("Unable to get direntry").path();
        if path.is_dir() {
            recurse_dir(v, path);
        } else if let Some(true) = path.extension().map(|v| v == "proto") {
            v.push(path);
        }
    }
}
fn main() {
    let mut files = Vec::new();
    recurse_dir(&mut files, "src/protocol/protobuf");

    prost_build::Config::new()
        .extern_path(".msg", "::ricq_core::pb::msg")
        .compile_protos(&files, &["src/protocol/protobuf", "src/protocol/core"])
        .expect("Cannot compile protobuf files");
}
