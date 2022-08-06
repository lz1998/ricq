#![allow(clippy::all)]

/*include!(concat!(env!("OUT_DIR"), "/pb.rs"));

macro_rules! add_includes {
    ($( $name:ident ),*) => {
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
    short_video
);
*/

include!("pb.rs");

pub mod cmd0x346;
pub mod cmd0x352;
pub mod cmd0x388;
pub mod cmd0x3bb;
pub mod cmd0x6ff;
pub mod cmd0x899;
pub mod longmsg;
pub mod msf;
pub mod msg;
pub mod msgtype0x210;
pub mod multimsg;
pub mod notify;
pub mod oidb;
pub mod online_status;
pub mod sig_act;
pub mod structmsg;
pub mod short_video;
pub mod profilecard;
pub mod guild;