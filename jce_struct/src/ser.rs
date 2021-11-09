use std::convert::TryInto;

use bytes::{BufMut, Bytes, BytesMut};

/// A struct to build a Jce package
pub struct JceMut(BytesMut);

// pub fn put_u8(&mut self, value: u8, tag: u8) -> &mut Self {
//     value.put(self, tag)
// }
macro_rules! puts {
    ($fn_name:ident, $t:ty) => {
        pub fn $fn_name(&mut self, value: $t, tag: u8) -> &mut Self {
            value.put(self, tag)
        }
    };
}

impl JceMut {
    /// new with 1024 capacity
    pub fn new() -> Self {
        JceMut(BytesMut::with_capacity(1024))
    }

    /// new with capacity
    pub fn with_capacity(size: usize) -> Self {
        JceMut(BytesMut::with_capacity(size))
    }

    /// freeze get Bytes
    pub fn freeze(self) -> Bytes {
        self.0.freeze()
    }

    /// write a head
    pub fn put_head(&mut self, t: u8, tag: u8) -> &mut Self {
        if tag < 15 {
            self.0.put_u8((tag << 4) | t);
        } else {
            self.0.put_u8(0xf0 | t);
            self.0.put_u8(tag);
        }
        self
    }

    /// write a JceObject(no head)
    pub fn put<V>(&mut self, value: V, tag: u8) -> &mut Self
    where
        V: JcePut,
    {
        value.put(self, tag)
    }

    puts!(put_u8, u8); // 0||12
    puts!(put_bool, bool); // 0
    puts!(put_i16, i16); // 1
    puts!(put_i32, i32); // 2
    puts!(put_i64, i64); // 3
    puts!(put_f32, f32); // 4
    puts!(put_f64, f64); // 5
    puts!(put_string, String); // 6||7

    pub fn put_map<K, V>(&mut self, value: std::collections::HashMap<K, V>, tag: u8) -> &mut Self
    where
        K: JcePut,
        V: JcePut,
    {
        value.put(self, tag)
    } // 8

    pub fn put_list<V>(&mut self, value: Vec<V>, tag: u8) -> &mut Self
    where
        V: JcePut,
    {
        value.put(self, tag)
    } // 9

    fn _put_jce_struct<S>(&mut self, value: S, tag: u8) -> &mut Self
    where
        S: JcePut,
    {
        self.put_head(10, tag);
        value.put(self, tag);
        self.put_head(11, 0)
    }
    // 10&&11
    // should impl in JcePut
    // this is just a demo

    puts!(put_bytes, Bytes); // 13
}

/// trait for Object can put to Jce Package
pub trait JcePut {
    /// put with given tag(with head)
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut;
    /// put raw_data(no head)
    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut;
}

impl JcePut for u8 {
    // byte
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        if self == 0 {
            jce_mut.put_head(12, tag); // empty tag
        } else {
            jce_mut.put_head(0, tag);
            jce_mut.0.put_u8(self);
        }
        jce_mut
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for bool {
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        jce_mut.put_u8(if self { 1 } else { 0 }, tag)
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for i16 {
    // Short
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        if self >= -128 && self <= 127 {
            jce_mut.put_u8(self.to_le_bytes()[0], tag)
        } else {
            jce_mut.put_head(1, tag);
            jce_mut.0.put_i16(self);
            jce_mut
        }
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for i32 {
    // Int
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        if let Ok(value) = self.try_into() {
            jce_mut.put_i16(value, tag)
        } else {
            jce_mut.put_head(2, tag);
            jce_mut.0.put_i32(self);
            jce_mut
        }
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for i64 {
    // Long
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        if let Ok(value) = self.try_into() {
            jce_mut.put_i32(value, tag)
        } else {
            jce_mut.put_head(3, tag);
            jce_mut.0.put_i64(self);
            jce_mut
        }
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for f32 {
    // Float
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        jce_mut.put_head(4, tag);
        jce_mut.0.put_f32(self);
        jce_mut
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for f64 {
    // Double
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        jce_mut.put_head(5, tag);
        jce_mut.0.put_f64(self);
        jce_mut
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for String {
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        let len = self.len();
        if len < 256 {
            jce_mut.put_head(6, tag);
            jce_mut.0.put_u8(len.try_into().unwrap());
            jce_mut.0.extend(self.as_bytes());
            jce_mut
        } else {
            jce_mut.put_head(7, tag);
            jce_mut.0.put_i32(len.try_into().unwrap());
            jce_mut.0.extend(self.as_bytes());
            jce_mut
        }
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl<K, V> JcePut for std::collections::HashMap<K, V>
where
    K: JcePut,
    V: JcePut,
{
    // Map
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        jce_mut.put_head(8, tag);
        if self.is_empty() {
            jce_mut.put_i32(0, 0);
            jce_mut
        } else {
            jce_mut.put_i32(self.len().try_into().unwrap(), 0);
            for (k, v) in self {
                k.put(jce_mut, 0);
                v.put(jce_mut, 1);
            }
            jce_mut
        }
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl<V> JcePut for Vec<V>
where
    V: JcePut,
{
    // List
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        jce_mut.put_head(9, tag);
        if self.is_empty() {
            jce_mut.put_i32(0, 0);
            jce_mut
        } else {
            jce_mut.put_i32(self.len().try_into().unwrap(), 0);
            for v in self {
                v.put(jce_mut, 0);
            }
            jce_mut
        }
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

impl JcePut for Bytes {
    // Bytes
    fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
        jce_mut.put_head(13, tag);
        jce_mut.put_head(0, 0);
        jce_mut.put_i32(self.len().try_into().unwrap(), 0);
        jce_mut.0.put(self);
        jce_mut
    }

    fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
        jce_mut
    }
}

pub trait JceToObject {
    fn build(self) -> Bytes;
}

impl<T> JceToObject for T
where
    T: JcePut,
{
    fn build(self) -> Bytes {
        let mut jce_mut = JceMut::with_capacity(1024);
        self.put_raw(&mut jce_mut);
        jce_mut.freeze()
    }
}

#[test]
fn just_test() {
    let a: i16 = -126;
    let b: u8 = a.to_le_bytes()[0];
    print!("{}", b);
}
