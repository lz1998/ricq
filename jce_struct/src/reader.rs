use std::collections::{BTreeMap, HashMap};

use bytes::Bytes;

use crate::Jce;
use crate::JceGet;

pub type JceMap = HashMap<JceMapKey, JceObject>;
pub type JceList = Vec<JceObject>;
pub type JceStruct = BTreeMap<u8, JceObject>;

#[derive(Debug)]
pub enum JceObject {
    U8(u8), // 0
    // Bool(bool), can't tell bool or u8 in here
    I16(i16),            // 1
    I32(i32),            // 2
    I64(i64),            // 3
    F32(f32),            // 4
    F64(f64),            // 5
    ShortString(String), // 6
    LongString(String),  // 7
    Map(JceMap),         // 8
    List(JceList),       // 9
    Struct(JceStruct),   // 10&&11
    Empty,               // 12
    Bytes(Bytes),        // 13
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum JceMapKey {
    U8(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    ShortString(String),
    LongString(String),
}

impl JceGet for JceObject {
    fn empty() -> Self {
        Self::Empty
    }

    fn read(jce: &mut Jce) -> Self {
        match jce.t {
            0 => Self::U8(u8::read(jce)), // can't tell bool or u8 in here
            1 => Self::I16(i16::read(jce)),
            2 => Self::I32(i32::read(jce)),
            3 => Self::I64(i64::read(jce)),
            4 => Self::F32(f32::read(jce)),
            5 => Self::F64(f64::read(jce)),
            6 => Self::ShortString(String::read(jce)),
            7 => Self::LongString(String::read(jce)),
            8 => Self::Map(HashMap::<JceMapKey, Self>::read(jce)),
            9 => Self::List(Vec::<Self>::read(jce)),
            10 => Self::Struct(JceStruct::read(jce)),
            12 => Self::empty(),
            13 => Self::Bytes(Bytes::read(jce)),
            _ => panic!("unkouwn type {}", jce.t),
        }
    }
}

impl JceGet for JceMapKey {
    fn empty() -> Self {
        panic!()
    }

    fn read(jce: &mut Jce) -> Self {
        match jce.t {
            0 => Self::U8(u8::read(jce)),
            1 => Self::I16(i16::read(jce)),
            2 => Self::I32(i32::read(jce)),
            3 => Self::I64(i64::read(jce)),
            6 => Self::ShortString(String::read(jce)),
            7 => Self::LongString(String::read(jce)),
            _ => panic!("error key type {}", jce.t),
        }
    }
}

impl JceGet for BTreeMap<u8, JceObject> {
    fn empty() -> Self {
        BTreeMap::new()
    }

    fn read(jce: &mut Jce) -> Self {
        let mut jce_struct = Self::empty();
        while jce.has_remaining() {
            jce.read_head();
            if jce.t != 11 {
                jce_struct.insert(jce.tag, JceObject::read(jce));
            } else {
                break; // end of struct
            }
        }
        jce_struct
    }
}

pub trait FromJceObject {
    fn read_from_obj(jce_obj: JceObject) -> Self;
}

impl FromJceObject for u8 {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::U8(v) = jce_obj {
            v
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl FromJceObject for bool {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::U8(v) = jce_obj {
            if v == 1 {
                true
            } else if v == 0 {
                false
            } else {
                panic!("type error, readding {:?}", jce_obj)
            }
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl FromJceObject for i16 {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::I16(v) = jce_obj {
            v
        } else {
            u8::read_from_obj(jce_obj) as i16
        }
    }
}

impl FromJceObject for i32 {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::I32(v) = jce_obj {
            v
        } else {
            i16::read_from_obj(jce_obj) as i32
        }
    }
}

impl FromJceObject for i64 {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::I64(v) = jce_obj {
            v
        } else {
            i32::read_from_obj(jce_obj) as i64
        }
    }
}

impl FromJceObject for f32 {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::F32(v) = jce_obj {
            v
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl FromJceObject for f64 {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::F64(v) = jce_obj {
            v
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl FromJceObject for String {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::ShortString(v) = jce_obj {
            v
        } else if let JceObject::LongString(v) = jce_obj {
            v
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl<V> FromJceObject for HashMap<JceMapKey, V>
where
    V: FromJceObject,
{
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::Map(m) = jce_obj {
            let mut r = HashMap::new();
            for (k, v) in m {
                r.insert(k, V::read_from_obj(v));
            }
            r
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl<T> FromJceObject for Vec<T>
where
    T: FromJceObject,
{
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::List(l) = jce_obj {
            let mut r = vec![];
            for obj in l {
                r.push(T::read_from_obj(obj))
            }
            r
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

impl FromJceObject for Bytes {
    fn read_from_obj(jce_obj: JceObject) -> Self {
        if let JceObject::Bytes(b) = jce_obj {
            b
        } else {
            panic!("type error, readding {:?}", jce_obj)
        }
    }
}

pub trait FromJce {
    fn read_from_jce(jce: Jce) -> Self;
}

impl<T> FromJce for T
where
    T: FromJceObject,
{
    fn read_from_jce(mut jce: Jce) -> Self {
        let jce_struct = JceStruct::read(&mut jce);
        T::read_from_obj(JceObject::Struct(jce_struct))
    }
}
