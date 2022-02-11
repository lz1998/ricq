use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "src/pb/data.proto",
            "src/pb/cmd0x388/cmd0x388.proto",
            "src/pb/cmd0x3bb/cmd0x3bb.proto",
            "src/pb/cmd0x6ff/smbcmd0x519.proto",
            "src/pb/cmd0x6ff/subcmd0x501.proto",
            "src/pb/msf/register_proxy.proto",
            "src/pb/msg/head.proto",
            "src/pb/msg/msg.proto",
            "src/pb/msg/objmsg.proto",
            "src/pb/msg/report.proto",
            "src/pb/msg/TextMsgExt.proto",
            "src/pb/msgtype0x210/subMsgType0x27.proto",
            "src/pb/notify/group0x857.proto",
            "src/pb/oidb/oidb.proto",
            "src/pb/oidb/oidb0x8a7.proto",
            "src/pb/oidb/oidb0x8fc.proto",
            "src/pb/oidb/oidb0x88d.proto",
            "src/pb/oidb/oidb0x758.proto",
            "src/pb/oidb/oidb0x769.proto",
            "src/pb/oidb/oidb0x990.proto",
            "src/pb/oidb/oidb0xb77.proto",
            "src/pb/oidb/oidb0xe07.proto",
            "src/pb/oidb/oidb0xeac.proto",
            "src/pb/profilecard/busi.proto",
            "src/pb/profilecard/gate.proto",
            "src/pb/structmsg/structmsg.proto",
        ],
        &["src/pb"],
    )?;
    Ok(())
}
