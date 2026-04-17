mod gf256;

struct State {
    data: [[u8; 4]; 4],
}

impl From<&[u8; 16]> for State {
    fn from(data: &[u8; 16]) -> State {
        State {
            data: [
                [data[0], data[1], data[2], data[3]],
                [data[4], data[5], data[6], data[7]],
                [data[8], data[9], data[10], data[11]],
                [data[12], data[13], data[14], data[15]],
            ],
        }
    }
}

impl From<State> for [u8; 16] {
    fn from(s: State) -> Self {
        let mut res = [0u8; 16];
        res[0..4].copy_from_slice(&s.data[0]);
        res[4..8].copy_from_slice(&s.data[1]);
        res[8..12].copy_from_slice(&s.data[2]);
        res[12..16].copy_from_slice(&s.data[3]);
    }
    res
}

impl State {

    pub fn get(&self, i: usize, j: usize) -> u8 {
        self.data[i][j]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut u8 {
        &mut self.data[i][j]
    }

    pub fn add_round_key(&mut self, w: &[u8; 4]) -> &mut Self {
        for c in 0..4 {
            let s0c = self.get(0, c);
            let s1c = self.get(1, c);
            let s2c = self.get(2, c);
            let s3c = self.get(3, c);
            self.data.get_mut(0, c) = s0c ^ w[c];
            self.data.get_mut(1, c) = s1c ^ w[c];
            self.data.get_mut(2, c) = s2c ^ w[c];
            self.data.get_mut(3, c) = s3c ^ w[c];
        } 
        self
    }

    pub fn sub_bytes(&mut self) -> &mut Self {
        self.data = self.data.map(|row| row.map(sbox));
        self
    }

    pub fn shift_rows(&mut self) -> &mut Self {
        for i in 1..4 {
            for j in 0..4 {
                self.data[i][j] = self.data[i][(j + 1) % 4];
            }
        }
        self
    }

    pub fn mix_columns(&mut self) -> &mut Self {
        let a: [u8; 4] = [0x02, 0x01, 0x01, 0x03];
        for c in 0..4 {
            let s0c = self.get(0, c);
            let s1c = self.get(1, c);
            let s2c = self.get(2, c);
            let s3c = self.get(3, c);
            self.get_mut(0, c) = gf256::mul(a[0], s0c) ^ gf256::mul(a[2], s1c) ^ s2c ^ s3c;
            self.get_mut(1, c) = s0c ^ gf256::mul(a[0], s1c) ^ gf256::mul(a[2], s2c) ^ s3c;
            self.get_mut(2, c) = s0c ^ s1c ^ gf256::mul(a[0], s2c) ^ gf256::mul(a[2], s3c);
            self.get_mut(3, c) = gf256::mul(a[2], s0c) ^ s1c ^ s2c ^ gf256::mul(a[0], s3c);
        }
        self
    }

    fn sbox(byte: u8) -> u8 {
        let c = 0b01100011u8;
        let mut b = gf256::inverse(byte);
        b ^ b.rotate_left(1) ^ b.rotate_left(2)
            ^ b.rotate_left(3) ^ b.rotate_left(4) ^ c
    }

    fn inverse_sbox(byte: u8) -> u8 {
        let c = 0b00000101u8;
        let b = gf256::inverse(byte);
        b.rotate_left(1) ^ b.rotate_left(3) ^ b.rotate_left(6) ^ c
    }
}

fn key_expansion(key: &[u8]) -> [u8;]


pub fn aes_128(in: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let k = key_expansion(key);
    let res = cipher(in, 10, k); 
}

pub fn aes_192(in: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let k = key_expansion(key);
    let res = cipher(in, 12, k);
}

pub fn aes_256(in: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let k = key_expansion(key);
    let res = cipher(in, 16, k);
}

fn cipher(in: &[u8; 16], nr: u32, w: &[u8]) -> [u8; 16] {
   let mut s = State::from(in);
   s.add_round_key(w[0..3]);
   for round in 1..nr-1 {
        s.sub_bytes()
            .shift_rows()
            .mix_columns()
            .add_round_key(w[4*round..4*round+3]);
   }
   s.sub_bytes()
       .shift_rows()
       .add_round_key(w[4*nr..4*nr+3]);
    s.into()
}
