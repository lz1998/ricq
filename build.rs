use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/pb/data.proto"], &["src/pb"])?;
    Ok(())
}