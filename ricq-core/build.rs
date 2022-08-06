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

    prost_build::Config::new().compile_protos(&v, &["src/pb"])?;
    let dir = env::var_os("OUT_DIR").unwrap();
    let dir = Path::new(&dir);

    let mut out = PathBuf::new();
    out.push("src");
    out.push("pb");

    for f in fs::read_dir(dir)? {
        if let Ok(entry) = f {
            let p = entry.path();

            if p.is_file() {
                let name = p.file_stem().unwrap();
                match name.to_str() {
                    Some("_") => {
                        continue;
                    },
                    Some("pb") => {
                        fs::copy(&p, "src/pb/pb.rs").expect("Cannot copy pb.rs");
                        continue;
                    },
                    _ => {
                    }
                }
                out.push(name);
                out.push("mod.rs");
                fs::copy(&p, &out).expect(&format!("Cannot copy file: {:?}", out));
                out.pop();
                out.pop();
            }
        }
    };

    //prost_build::compile_protos(&v, &["src/pb"])?;
    Ok(())
}