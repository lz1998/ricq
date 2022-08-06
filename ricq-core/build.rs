use std::{env, fs};
use std::io::Result;
use std::path::{Path, PathBuf};

fn recursion(v: &mut Vec<String>, dir: impl AsRef<Path>) -> Result<()> {
    let rd = std::fs::read_dir(dir)?;
    for x in rd {
        let de = x?;
        let path = de.path();
        if path.is_dir() {
            recursion(v, path)?;
        } else {
            let path = path.into_os_string().into_string().expect("path error");
            if path.ends_with(".proto") {
                v.push(path);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut v = Vec::<String>::new();
    recursion(&mut v, "src/pb")?;

    prost_build::compile_protos(&v, &["src/pb"])?;
    Ok(())
}