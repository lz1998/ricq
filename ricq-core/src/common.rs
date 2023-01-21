use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn group_code2uin(code: i64) -> i64 {
    let mut left = code / 1000000;
    if (0..=10).contains(&left) {
        left += 202
    } else if (11..=19).contains(&left) {
        left += 469
    } else if (20..=66).contains(&left) {
        left += 2080
    } else if (67..=156).contains(&left) {
        left += 1943
    } else if (157..=209).contains(&left) {
        left += 1990
    } else if (210..=309).contains(&left) {
        left += 3890
    } else if (310..=335).contains(&left) {
        left += 3490
    } else if (336..=386).contains(&left) {
        //335 336不确定
        left += 2265
    } else if (387..=499).contains(&left) {
        left += 3490
    }
    left * 1000000 + code % 1000000
}

pub fn group_uin2code(uin: i64) -> i64 {
    let mut left = uin / 1000000;
    if (202..=212).contains(&left) {
        left -= 202
    } else if (480..=488).contains(&left) {
        left -= 469
    } else if (2100..=2146).contains(&left) {
        left -= 2080
    } else if (2010..=2099).contains(&left) {
        left -= 1943
    } else if (2147..=2199).contains(&left) {
        left -= 1990
    } else if (2600..=2651).contains(&left) {
        left -= 2265
    } else if (3800..=3989).contains(&left) {
        left -= 3490
    } else if (4100..=4199).contains(&left) {
        left -= 3890
    }
    left * 1000000 + uin % 1000000
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RQAddr(pub u32, pub u16);

impl From<RQAddr> for SocketAddr {
    fn from(addr: RQAddr) -> Self {
        let mut ip: [u8; 4] = addr.0.to_be_bytes();
        ip.reverse();
        SocketAddr::new(Ipv4Addr::from(ip).into(), addr.1)
    }
}

impl From<SocketAddr> for RQAddr {
    fn from(addr: SocketAddr) -> Self {
        let IpAddr::V4(ip) = addr.ip() else { panic!("is not ipv4") };
        // ip.octets() returns little-endian
        Self(u32::from_le_bytes(ip.octets()), addr.port())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_code2uin() {
        let uin = group_code2uin(335783090);
        assert_eq!(uin, 3825783090);
    }
    #[test]
    fn test_group_uin2code() {
        let code = group_uin2code(3825783090);
        assert_eq!(code, 335783090);
    }
}
