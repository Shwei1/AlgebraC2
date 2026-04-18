#![allow(dead_code)]
use crate::gf256;

struct State {
    data: [[u8; 4]; 4],
}

impl From<&[u8; 16]> for State {
    fn from(data: &[u8; 16]) -> State {
        State {
            data: [
                [data[0], data[4], data[8], data[12]],
                [data[1], data[5], data[9], data[13]],
                [data[2], data[6], data[10], data[14]],
                [data[3], data[7], data[11], data[15]],
            ],
        }
    }
}

impl From<State> for [u8; 16] {
    fn from(s: State) -> Self {
        let mut res = [0u8; 16];
        for col in 0..4 {
            for row in 0..4 {
                res[col * 4 + row] = s.data[row][col];
            }
        }
        res
    }
}

impl State {

    fn get(&self, i: usize, j: usize) -> u8 {
        self.data[i][j]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut u8 {
        &mut self.data[i][j]
    }

    fn add_round_key(&mut self, w: &[u8]) -> &mut Self {
        assert_eq!(w.len(), 16);
        for col in 0..4 {
            for row in 0..4 {
                *self.get_mut(row, col) ^= w[col * 4 + row];
            }
        }
        self
    }   

    fn sub_bytes(&mut self) -> &mut Self {
        self.data = self.data.map(|row| row.map(|x| Self::sbox(x)));
        self
    }

    fn shift_rows(&mut self) -> &mut Self {
        let mut new_data = self.data;
        for i in 1..4 {
            for j in 0..4 {
                new_data[i][j] = self.data[i][(j + i) % 4];
            }
        }
        self.data = new_data;
        self
    }

    fn mix_columns(&mut self) -> &mut Self {
        let a: [u8; 4] = [0x02, 0x01, 0x01, 0x03];
        for c in 0..4 {
            let s0c = self.get(0, c);
            let s1c = self.get(1, c);
            let s2c = self.get(2, c);
            let s3c = self.get(3, c);
            *self.get_mut(0, c) = gf256::mul(a[0], s0c) ^ gf256::mul(a[3], s1c) ^ s2c ^ s3c;
            *self.get_mut(1, c) = s0c ^ gf256::mul(a[0], s1c) ^ gf256::mul(a[3], s2c) ^ s3c;
            *self.get_mut(2, c) = s0c ^ s1c ^ gf256::mul(a[0], s2c) ^ gf256::mul(a[3], s3c);
            *self.get_mut(3, c) = gf256::mul(a[3], s0c) ^ s1c ^ s2c ^ gf256::mul(a[0], s3c);
        }
        self
    }


    fn sbox(byte: u8) -> u8 {
        let c = 0b01100011u8;
        let b = gf256::inverse(byte);
        b
            ^ (b << 1) ^ (b >> 7)
            ^ (b << 2) ^ (b >> 6)
            ^ (b << 3) ^ (b >> 5)
            ^ (b << 4) ^ (b >> 4)
            ^ c
}

// TODO: rewrite all inverse functions

    fn inv_sbox(byte: u8) -> u8 {
        let c = 0b00000101u8;
        let b = gf256::inverse(byte);
        b.rotate_left(1) ^ b.rotate_left(3) ^ b.rotate_left(6) ^ c
    }

    fn inv_shift_rows(&mut self) -> &mut Self {
        for i in 1..4 {
            for j in 0..4 {
                self.data[i][j] = self.data[i][(j + 4 - i) % 4];
            }
        }
        self
    }

    fn inv_sub_bytes(&mut self) -> &mut Self {
        self.data = self.data.map(|row| row.map(|x| Self::inv_sbox(x)));
        self
    }

    fn inv_mix_columns(&mut self) -> &mut Self {
        let a: [u8; 4] = [0x0e, 0x09, 0x0d, 0x0b];
        for c in 0..4 {
            let s0c = self.get(0, c);
            let s1c = self.get(1, c);
            let s2c = self.get(2, c);
            let s3c = self.get(3, c);
            *self.get_mut(0, c) = gf256::mul(a[0], s0c) ^ gf256::mul(a[3], s1c) ^ gf256::mul(a[2], s2c) ^ gf256::mul(a[1], s3c);
            *self.get_mut(1, c) = gf256::mul(a[1], s0c) ^ gf256::mul(a[0], s1c) ^ gf256::mul(a[3], s2c) ^ gf256::mul(a[2], s3c);
            *self.get_mut(2, c) = gf256::mul(a[2], s0c) ^ gf256::mul(a[1], s1c) ^ gf256::mul(a[0], s2c) ^ gf256::mul(a[3], s3c);
            *self.get_mut(3, c) = gf256::mul(a[3], s0c) ^ gf256::mul(a[2], s1c) ^ gf256::mul(a[3], s2c) ^ gf256::mul(a[0], s3c);
        }
        self
    }


}

fn key_expansion(key: &[u8]) -> Vec<u8> {
    fn xor(a: &[u8; 4], b: &[u8; 4]) -> [u8; 4] {
        std::array::from_fn(|i| a[i] ^ b[i])
    }
    let nk = key.len() / 4;
    let nr = match key.len() {
        16 => 10,
        24 => 12,
        32 => 14,
        _ => panic!()
    };
    let rcon: [[u8; 4]; 10] = [[0x01, 0x00, 0x00, 0x00], [0x02, 0x00, 0x00, 0x00], 
                               [0x04, 0x00, 0x00, 0x00], [0x08, 0x00, 0x00, 0x00],
                               [0x10, 0x00, 0x00, 0x00], [0x20, 0x00, 0x00, 0x00],
                               [0x40, 0x00, 0x00, 0x00], [0x80, 0x00, 0x00, 0x00],
                               [0x1b, 0x00, 0x00, 0x00], [0x36, 0x00, 0x00, 0x00]
                              ];
    let mut i = 0usize;
    let mut w: Vec<[u8; 4]> = vec![[0u8; 4]; (nr+1)*4];
    while i <= nk-1 {
        w[i] = key[4*i..4*i+4].try_into().unwrap();
        i += 1;
    }
    while i <= 4*nr+3 {
        let mut temp = w[i-1];
        if i % nk == 0 {
            temp = xor(&sub_word(&rot_word(&temp)), &rcon[i / nk - 1]);
        } else if nk > 6 && i % nk == 4 {
            temp = sub_word(&temp);
        }
        w[i] = xor(&w[i-nk], &temp);
        i += 1;
    }
    w.into_iter().flatten().collect()
}

fn sub_word(a: &[u8; 4]) -> [u8; 4] {
    std::array::from_fn(|i| State::sbox(a[i]))
}

fn rot_word(a: &[u8; 4]) -> [u8; 4] {
    [a[1], a[2], a[3], a[0]]
}


pub fn aes_128(in_: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let k = key_expansion(key);
    let res = cipher(in_, 10, &k);
    res
}

pub fn aes_192(in_: &[u8; 16], key: &[u8; 24]) -> [u8; 16] {
    let k = key_expansion(key);
    let res = cipher(in_, 12, &k);
    res
}

pub fn aes_256(in_: &[u8; 16], key: &[u8; 32]) -> [u8; 16] {
    let k = key_expansion(key);
    let res = cipher(in_, 14, &k);
    res
}

fn cipher(in_: &[u8; 16], nr: u32, w: &[u8]) -> [u8; 16] {
   let mut s = State::from(in_);
   s.add_round_key(&w[0..16]);
   for round in 1..nr as usize {
        s.sub_bytes()
            .shift_rows()
            .mix_columns()
            .add_round_key(&w[16*round..16*(round+1)]);
   }
   s.sub_bytes()
       .shift_rows()
       .add_round_key(&w[16*nr as usize..16*(nr as usize +1)]);
    s.into()
}

// TODO: check for bugs
fn inv_cipher(in_: &[u8; 16], nr: u32, w: &[u8]) -> [u8; 16] {
    let mut s = State::from(in_);
    s.add_round_key(&w[16*nr as usize..16*(nr as usize+1)]);
    for round in (nr-1) as usize..1 {
        s.inv_shift_rows()
            .inv_sub_bytes()
            .add_round_key(&w[16*round..16*(round+1)])
            .inv_mix_columns();
    }
    s.inv_shift_rows()
        .inv_sub_bytes()
        .add_round_key(&w[0..16]);
    s.into()
}
