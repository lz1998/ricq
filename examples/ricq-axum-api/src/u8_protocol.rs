use ricq::version::Protocol;

pub trait U8Protocol {
    fn to_u8(&self) -> u8;
    fn from_u8(n: u8) -> Self;
}

impl U8Protocol for Protocol {
    fn to_u8(&self) -> u8 {
        match self {
            Protocol::AndroidPhone => 1,
            Protocol::AndroidWatch => 2,
            Protocol::MacOS => 3,
            Protocol::QiDian => 4,
            Protocol::IPad => 5,
        }
    }

    fn from_u8(n: u8) -> Self {
        match n {
            1 => Protocol::AndroidPhone,
            2 => Protocol::AndroidWatch,
            3 => Protocol::MacOS,
            4 => Protocol::QiDian,
            5 => Protocol::IPad,
            _ => Protocol::IPad,
        }
    }
}
