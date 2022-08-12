#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/pb.rs"));

macro_rules! add_includes {
    ($( $name:ident ),* $(,)?) => {
        $(
            pub mod $name {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($name), ".rs"));
            }
        )*
    };
}

add_includes!(
    cmd0x346,
    cmd0x352,
    cmd0x388,
    cmd0x3bb,
    cmd0x6ff,
    cmd0x899,
    longmsg,
    msf,
    msg,
    msgtype0x210,
    multimsg,
    notify,
    oidb,
    online_status,
    profilecard,
    sig_act,
    structmsg,
    short_video,
);
