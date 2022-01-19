use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "src/pb/data.proto",
            "src/pb/msg/msg.proto",
            "src/pb/msg/head.proto",
            "src/pb/msg/objmsg.proto",
            "src/pb/msg/report.proto",
            "src/pb/msg/TextMsgExt.proto",
            "src/pb/structmsg/structmsg.proto",
            "src/pb/cmd0x6ff/smbcmd0x519.proto",
            "src/pb/cmd0x6ff/subcmd0x501.proto",
            "src/pb/notify/group0x857.proto",
            "src/pb/msgtype0x210/subMsgType0x27.proto",
            "src/pb/oidb/oidb0x758.proto",
            "src/pb/oidb/oidb0x769.proto",
            "src/pb/oidb/oidb0x8a7.proto",
            "src/pb/oidb/oidb0x88d.proto",
            "src/pb/oidb/oidb0x990.proto",
            "src/pb/oidb/oidb.proto",
            "src/pb/msf/register_proxy.proto",
        ],
        &["src/pb"],
    )?;
    Ok(())
}
