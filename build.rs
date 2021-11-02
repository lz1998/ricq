use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&[
        "src/pb/data.proto",
        "src/pb/msg/msg.proto",
        "src/pb/msg/head.proto",
        "src/pb/msg/objmsg.proto",
        "src/pb/msg/report.proto",
        "src/pb/structmsg/structmsg.proto",
    ], &["src/pb"])?;
    Ok(())
}