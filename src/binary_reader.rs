use bytes::Buf;

pub trait BinaryReader {
    fn read_string(&mut self) -> String;
}

impl<B> BinaryReader for B
    where B: Buf {
    fn read_string(&mut self) -> String {
        let len = self.get_i32() as usize - 4;
        String::from_utf8(self.copy_to_bytes(len).to_vec()).unwrap()
    }
}