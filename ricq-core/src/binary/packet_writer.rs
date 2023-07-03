use bytes::*;
use std::marker::PhantomData;

pub trait PacketWriter<B: BufMut> {
    fn write(self, buf: &mut B);
}

pub trait PacketAppender<B: BufMut>: PacketWriter<B> + Sized {
    fn append<W: PacketWriter<B>>(self, w: W) -> impl PacketWriter<B> {
        |buf: &mut B| {
            self.write(buf);
            w.write(buf);
        }
    }
}

impl<F, B> PacketWriter<B> for F
where
    B: BufMut,
    F: FnOnce(&mut B),
{
    fn write(self, buf: &mut B) {
        self(buf)
    }
}

impl<B> PacketWriter<B> for &[u8]
where
    B: BufMut,
{
    fn write(self, buf: &mut B) {
        buf.put_slice(self);
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<B, L, R> PacketWriter<B> for Either<L, R>
where
    B: BufMut,
    L: PacketWriter<B>,
    R: PacketWriter<B>,
{
    fn write(self, buf: &mut B) {
        match self {
            Either::Left(l) => l.write(buf),
            Either::Right(r) => r.write(buf),
        }
    }
}

pub struct CounterWriter<B, W>
where
    B: BufMut,
    W: PacketWriter<B>,
{
    pub count: usize,
    pub writer: W,
    pub _mark: PhantomData<B>,
}

impl<B, T> PacketWriter<B> for CounterWriter<B, T>
where
    B: BufMut,
    T: PacketWriter<B>,
{
    fn write(self, buf: &mut B) {
        self.writer.write(buf)
    }
}

impl<B, T> PacketAppender<B> for CounterWriter<B, T>
where
    B: BufMut,
    T: PacketWriter<B>,
{
    fn append<W>(self, w: W) -> CounterWriter<B, impl PacketWriter<B>>
    where
        W: PacketWriter<B>,
    {
        CounterWriter {
            count: self.count + 1,
            writer: |buf: &mut B| {
                self.writer.write(buf);
                w.write(buf);
            },
            _mark: PhantomData,
        }
    }
}

impl<B: BufMut> Default for CounterWriter<B, fn(&mut B)> {
    fn default() -> Self {
        CounterWriter {
            count: 0,
            writer: |_| {},
            _mark: PhantomData,
        }
    }
}

impl<B, T> CounterWriter<B, T>
where
    B: BufMut,
    T: PacketWriter<B>,
{
    pub fn append_option<W>(self, w: Option<W>) -> CounterWriter<B, impl PacketWriter<B>>
    where
        W: PacketWriter<B>,
    {
        CounterWriter {
            count: self.count + if w.is_some() { 1 } else { 0 },
            writer: |buf: &mut B| {
                self.writer.write(buf);
                if let Some(w) = w {
                    w.write(buf)
                }
            },
            _mark: PhantomData,
        }
    }
}
// write length-value
pub trait WriteLV: BufMut {
    fn write_short_lv<W>(&mut self, w: W)
    where
        W: PacketWriter<Self>,
        Self: Sized;
}

macro_rules! impl_write_lv {
    () => {
        fn write_short_lv<W>(&mut self, w: W)
        where
            W: PacketWriter<Self>,
            Self: Sized,
        {
            let len_start = self.len();
            self.put_u16(0);
            let body_start = self.len();
            w.write(self);
            let body_len = (self.len() - body_start) as u16;
            (&mut self[len_start..len_start + 2]).put_u16(body_len);
        }
    };
}

impl WriteLV for Vec<u8> {
    impl_write_lv!();
}

impl WriteLV for BytesMut {
    impl_write_lv!();
}
