use std::{hash::Hash, panic};

use bytes::{Buf, Bytes};

use crate::reader::{JceList, JceMap, JceStruct};

/// Jce decoder
#[derive(Debug)]
pub struct Jce<'a> {
    pub b: &'a mut Bytes,
    pub t: u8,
    pub tag: u8,
    pub new: bool,
}

/// trait for JceStruct
pub trait JceGet {
    /// get Self form Jce
    fn read(jce: &mut Jce) -> Self;
    /// declera empty Self or panic if require
    fn empty() -> Self;
}

impl<'a> Jce<'a> {
    /// new Jce
    pub fn new(bytes: &'a mut Bytes) -> Self {
        Jce {
            b: bytes,
            t: 0,
            tag: 0,
            new: true,
        }
    }

    /// return Bytes.has_remaining
    pub fn has_remaining(&self) -> bool {
        self.b.has_remaining()
    }

    /// get a obj from Jce(impl JceGet)
    ///
    /// type should be declera when use this method
    pub fn get<T>(&mut self) -> T
        where
            T: JceGet,
    {
        if self.t != 12 {
            T::read(self)
        } else {
            T::empty()
        }
    }

    /// build a Jce from the bytes and read to a obj(impl JceGet)
    pub fn read_from_bytes<T>(bytes: &'a mut Bytes) -> T
        where
            T: JceGet,
    {
        let mut jce = Self::new(bytes);
        jce.get()
    }

    /// get a obj(impl JceGet) by given tag
    ///
    /// usually use to impl JceGet
    pub fn get_by_tag<T>(&mut self, tag: u8) -> T
        where
            T: JceGet,
    {
        if self.new {
            self.read_head();
            self.new = false
        }
        if self.tag != tag {
            self.go_to_tag(tag);
        }
        self.get()
    }

    /// read a head
    pub fn read_head(&mut self) {
        let byte = self.b.get_u8();
        let t = byte & 0xF;
        let mut tag = (byte & 0xF0) >> 4;
        if tag == 15 {
            let n_byte = self.b.get_u8();
            tag = n_byte & 0xFF
        };
        self.t = t;
        self.tag = tag;
    }

    /// go to the JceObject with givven tag(skip obj if need)
    pub fn go_to_tag(&mut self, tag: u8) {
        self.read_head();
        while self.tag != tag {
            self.skip_block();
            if self.has_remaining() {
                self.read_head();
            } else {
                panic!("getting tag out of range")
            }
        }
    }

    /// go to the JceObject with givven type(skip obj if need)
    pub fn go_to_type(&mut self, t: u8) {
        self.read_head();
        while self.t != t {
            self.skip_block();
            if self.has_remaining() {
                self.read_head();
            } else {
                panic!("getting type out of range")
            }
        }
    }

    /// skip a JceObject
    pub fn skip_block(&mut self) {
        match self.t {
            0 => {
                u8::read(self);
            }
            1 => {
                i16::read(self);
            }
            2 => {
                i32::read(self);
            }
            3 => {
                i64::read(self);
            }
            4 => {
                f32::read(self);
            }
            5 => {
                f64::read(self);
            }
            6 => {
                String::read(self);
            }
            7 => {
                String::read(self);
            }
            8 => {
                JceMap::read(self);
            }
            9 => {
                JceList::read(self);
            }
            10 => {
                JceStruct::read(self);
            }
            11 => {} // end of object
            12 => {} // empty type
            13 => {
                Bytes::read(self);
            }
            _ => panic!("skip unkown {}", self.t),
        }
    }

    /// read the 0x0b byte if need
    pub fn end_object(&mut self) {
        if self.b.remaining() > 0 {
            self.go_to_type(11);
        }
    }
}

macro_rules! impl_JceGet {
    ($t:ty, $fn_name:ident) => {
        impl JceGet for $t {
            fn read(jce: &mut Jce) -> Self {
                jce.b.$fn_name()
            }

            fn empty() -> Self {
                0 as Self
            }
        }
    };
}

impl JceGet for u8 {
    fn read(jce: &mut Jce) -> Self {
        if jce.t == 12 {
            Self::empty()
        } else {
            jce.b.get_u8()
        }
    }

    fn empty() -> Self {
        0
    }
}

impl JceGet for i16 {
    fn read(jce: &mut Jce) -> Self {
        if jce.t < 1 {
            u8::read(jce) as i16
        } else if jce.t == 1 {
            jce.b.get_i16()
        } else if jce.t == 12 {
            Self::empty()
        } else {
            panic!("can't parse tag:{} Jce to i16", jce.t)
        }
    }

    fn empty() -> Self {
        0
    }
}

impl JceGet for i32 {
    fn read(jce: &mut Jce) -> Self {
        if jce.t < 2 {
            i16::read(jce) as i32
        } else if jce.t == 2 {
            jce.b.get_i32()
        } else if jce.t == 12 {
            Self::empty()
        } else {
            panic!("can't parse tag:{} Jce to i32", jce.t)
        }
    }

    fn empty() -> Self {
        0
    }
}

impl JceGet for i64 {
    fn read(jce: &mut Jce) -> Self {
        if jce.t < 3 {
            i32::read(jce) as i64
        } else if jce.t == 3 {
            jce.b.get_i64()
        } else if jce.t == 12 {
            Self::empty()
        } else {
            panic!("can't parse tag:{} Jce to i64", jce.t)
        }
    }

    fn empty() -> Self {
        0
    }
}
// impl_JceGet!(u8, get_u8); // 0
// impl_JceGet!(i16, get_i16); // 1
// impl_JceGet!(i32, get_i32); // 2
// impl_JceGet!(i64, get_i64); // 3
impl_JceGet!(f32, get_f32); // 4
impl_JceGet!(f64, get_f64); // 5

impl JceGet for String {
    // 6||7
    fn read(jce: &mut Jce) -> Self {
        let len = if jce.t == 6 {
            jce.b.get_u8() as i32
        } else {
            jce.b.get_i32()
        };
        let bytes = jce.b.copy_to_bytes(len as usize);
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn empty() -> Self {
        String::default()
    }
}

impl<K, V> JceGet for std::collections::HashMap<K, V>
    where
        K: JceGet + Eq + Hash,
        V: JceGet,
{
    // 8
    fn read(jce: &mut Jce) -> Self {
        let mut jce = Jce::new(jce.b);
        let len: i32 = jce.get_by_tag(0);
        let mut map = std::collections::HashMap::new();
        let mut jce = Jce::new(jce.b);
        for _ in 0..len {
            let k = jce.get_by_tag(0);
            let v = jce.get_by_tag(1);
            map.insert(k, v);
            jce.new = true;
        }
        map
    }

    fn empty() -> Self {
        panic!("jce get empty, should have a map")
    }
}

impl<V> JceGet for Vec<V>
    where
        V: JceGet,
{
    // 9
    fn read(jce: &mut Jce) -> Self {
        let mut jce = Jce::new(jce.b);
        let len: i32 = jce.get_by_tag(0);
        let mut list = vec![];
        for _ in 0..len {
            let mut jce = Jce::new(jce.b);
            list.push(jce.get_by_tag(0));
        }
        list
    }

    fn empty() -> Self {
        panic!("jce get empty, should have a set")
    }
}

impl JceGet for Bytes {
    // 13
    fn read(jce: &mut Jce) -> Self {
        jce.b.get_u8();
        jce.go_to_tag(0);
        let len = i32::read(jce);
        jce.b.copy_to_bytes(len as usize)
    }

    fn empty() -> Self {
        panic!("jce get empty, should have a bytes")
    }
}
