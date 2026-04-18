#![allow(dead_code)]

pub fn inverse(x: u8) -> u8 {
    if x == 0u8 {
        return 0u8;
    }
    let x0 = 1u8;
    let x2 = mul(x, x);
    let x4 = mul(x2, x2);
    let x8 = mul(x4, x4);
    let x16 = mul(x8, x8);
    let x32 = mul(x16, x16);
    let x64 = mul(x32, x32);
    let x128 = mul(x64, x64);
    mul(x128, mul(x64, mul(x32, mul(x16, mul(x8, mul(x4, mul(x2, x0)))))))
}

pub fn add(x: u8, y: u8) -> u8 {
    x ^ y
}


pub fn mul(mut x: u8, mut y: u8) -> u8 {
    let mut res = 0u8;
    while y != 0u8 {
        if (y & 1u8) != 0u8 {
            res ^= x;
        }
        let c = x & 0x80u8;
        x <<= 1u8;
        if c != 0 {
            x ^= 0x1Bu8;
        }
        y >>= 1u8;
    }
    res
}

