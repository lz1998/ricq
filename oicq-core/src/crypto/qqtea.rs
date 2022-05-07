// copy from https://github.com/zkonge/rtea
use byteorder::{BigEndian, ByteOrder};
use rand::{thread_rng, RngCore};

use tea::{GenericArray, Tea16};

pub fn qqtea_encrypt(text: &[u8], key: &[u8]) -> Vec<u8> {
    let fill_count = 9 - (text.len() + 1) % 8;

    let mut plaintext = vec![0u8; 1 + fill_count + text.len() + 7];
    let plaintext_len = plaintext.len();

    plaintext[0] = (fill_count as u8 - 2) | 0xF8;
    if cfg!(debug_assertions) {
        //这里是为了和pytea对拍，填充220
        plaintext[1..fill_count + 1].fill(220);
    } else {
        thread_rng().fill_bytes(&mut plaintext[1..fill_count + 1]);
    }
    plaintext[fill_count + 1..plaintext_len - 7].copy_from_slice(text);

    let mut work_block: Vec<u64> = vec![0; plaintext.len() / 8];

    BigEndian::read_u64_into(&plaintext, &mut work_block);

    let mut iv1 = 0u64;
    let mut iv2 = 0u64;
    let mut holder: u64;

    let cipher = Tea16::new(GenericArray::from_slice(key));

    for block in work_block.iter_mut() {
        holder = *block ^ iv1;

        iv1 = cipher.encrypt(holder);

        iv1 ^= iv2;

        iv2 = holder;

        *block = iv1;
    }

    BigEndian::write_u64_into(&work_block, &mut plaintext);

    plaintext
}

pub fn qqtea_decrypt(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut work_block: Vec<u64> = vec![0; text.len() / 8];

    BigEndian::read_u64_into(text, &mut work_block);

    let mut iv1 = 0u64;
    let mut iv2 = 0u64;
    let mut holder: u64;
    let mut tmp_block: u64;

    let cipher = Tea16::new(GenericArray::from_slice(key));

    for block in work_block.iter_mut() {
        tmp_block = *block ^ iv2;

        tmp_block = cipher.decrypt(tmp_block);

        iv2 = tmp_block;

        holder = tmp_block ^ iv1;

        iv1 = *block;

        *block = holder;
    }

    let mut result = vec![0u8; text.len()];

    BigEndian::write_u64_into(&work_block, &mut result);

    let begin_pos = ((result[0] as usize) & 7) + 3;
    let end_pos = result.len() - 7;

    result[begin_pos..end_pos].to_owned()
}

mod tea {
    use byteorder::{BigEndian, ByteOrder};
    pub use generic_array::{
        typenum::{U16, U8},
        GenericArray,
    };

    const TEA_DELTA: u32 = 0x9E3779B9;

    pub struct Tea16 {
        key: [u32; 4],
    }

    impl Tea16 {
        #[inline]
        pub fn encrypt(&self, n: u64) -> u64 {
            let mut sum: u32 = 0;
            let (mut x, mut y) = ((n >> 32) as u32, n as u32);
            let k0 = self.key[0];
            let k1 = self.key[1];
            let k2 = self.key[2];
            let k3 = self.key[3];

            for _ in 0..16 {
                sum = sum.wrapping_add(TEA_DELTA);
                x = x.wrapping_add(
                    k0.wrapping_add(y << 4) ^ y.wrapping_add(sum) ^ k1.wrapping_add(y >> 5),
                );
                y = y.wrapping_add(
                    k2.wrapping_add(x << 4) ^ x.wrapping_add(sum) ^ k3.wrapping_add(x >> 5),
                );
            }

            ((x as u64) << 32) | y as u64
        }

        #[inline]
        pub fn decrypt(&self, n: u64) -> u64 {
            let mut sum: u32 = TEA_DELTA << 4;
            let (mut x, mut y) = ((n >> 32) as u32, n as u32);
            let k0 = self.key[0];
            let k1 = self.key[1];
            let k2 = self.key[2];
            let k3 = self.key[3];

            for _ in 0..16 {
                y = y.wrapping_sub(
                    k2.wrapping_add(x << 4) ^ x.wrapping_add(sum) ^ k3.wrapping_add(x >> 5),
                );
                x = x.wrapping_sub(
                    k0.wrapping_add(y << 4) ^ y.wrapping_add(sum) ^ k1.wrapping_add(y >> 5),
                );
                sum = sum.wrapping_sub(TEA_DELTA);
            }

            ((x as u64) << 32) | y as u64
        }

        #[inline]
        pub fn new(key: &GenericArray<u8, U16>) -> Self {
            Self {
                key: [
                    BigEndian::read_u32(&key[0..4]),
                    BigEndian::read_u32(&key[4..8]),
                    BigEndian::read_u32(&key[8..12]),
                    BigEndian::read_u32(&key[12..16]),
                ],
            }
        }
    }

    #[allow(dead_code)]
    pub fn tea16_encrypt(text: &mut [u8], key: &[u8]) {
        let key: &GenericArray<u8, U16> = GenericArray::from_slice(key);

        let mut n = BigEndian::read_u64(text);

        n = Tea16::new(key).encrypt(n);

        BigEndian::write_u64(text, n);
    }

    #[allow(dead_code)]
    pub fn tea16_decrypt(text: &mut [u8], key: &[u8]) {
        let key: &GenericArray<u8, U16> = GenericArray::from_slice(key);

        let mut n = BigEndian::read_u64(text);

        n = Tea16::new(key).decrypt(n);

        BigEndian::write_u64(text, n);
    }
}
