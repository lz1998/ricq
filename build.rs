use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "src/engine/pb/data.proto",
            "src/engine/pb/msg/msg.proto",
            "src/engine/pb/msg/head.proto",
            "src/engine/pb/msg/objmsg.proto",
            "src/engine/pb/msg/report.proto",
            "src/engine/pb/msg/TextMsgExt.proto",
            "src/engine/pb/structmsg/structmsg.proto",
            "src/engine/pb/cmd0x6ff/smbcmd0x519.proto",
            "src/engine/pb/cmd0x6ff/subcmd0x501.proto",
            "src/engine/pb/notify/group0x857.proto",
            "src/engine/pb/msgtype0x210/subMsgType0x27.proto",
            "src/engine/pb/oidb/oidb0x769.proto",
            "src/engine/pb/oidb/oidb0x88d.proto",
            "src/engine/pb/oidb/oidb.proto",
            "src/engine/pb/msf/register_proxy.proto",
        ],
        &["src/engine/pb"],
    )?;
    Ok(())
}
