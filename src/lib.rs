pub mod binary;
pub mod client;
mod config;
pub mod crypto;
pub mod hex;
pub mod jce;

pub use config::Config;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/pb.rs"));

    pub mod structmsg {
        include!(concat!(env!("OUT_DIR"), "/structmsg.rs"));
    }

    pub mod msg {
        include!(concat!(env!("OUT_DIR"), "/msg.rs"));
    }

    pub mod cmd0x6ff {
        include!(concat!(env!("OUT_DIR"), "/cmd0x6ff.rs"));
    }

    pub mod notify {
        include!(concat!(env!("OUT_DIR"), "/notify.rs"));
    }

    pub mod msgtype0x210 {
        include!(concat!(env!("OUT_DIR"), "/msgtype0x210.rs"));
    }

    pub mod oidb {
        include!(concat!(env!("OUT_DIR"), "/oidb.rs"));
    }

    pub mod msf {
        include!(concat!(env!("OUT_DIR"), "/msf.rs"));
    }
}
