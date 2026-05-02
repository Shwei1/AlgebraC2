use crypto_bigint::{U512, Limb};
use crate::utils::pow_mod;

pub mod oaep;

#[derive(Debug)]
pub struct PublicKey {
    pub n: U512,
    pub e: U512,
}


#[derive(Debug)]
pub enum PrivateKey {
    Pair { n: U512, d: U512},
    Quintuple { p: U512, q: U512, dp: U512, dq: U512, q_inv: U512, rdt: Vec<(U512, U512, U512)>},
}


pub fn i2osp(x: U512, x_len: usize) -> Result<Vec<u8>, &'static str> {
    let mut words = Limb::array_as_words(x.as_limbs()).to_vec();
    words.reverse(); 
    let bytes: [u8; 64] = words
        .into_iter()
        .flat_map(|word| word.to_be_bytes())
        .collect::<Vec<u8>>()
        .try_into()
        .expect("Wrong data length");
    if x_len > 64 {
        return Err("Wrong requested size");
    }

    Ok(bytes[64-x_len..].to_vec())
}


pub fn os2ip(x: &[u8]) -> U512 {
    let res: [u8; 64] = x.try_into().unwrap();
    let mut limbs: [u64; 8] = core::array::from_fn(|i| {
        u64::from_be_bytes(res[i*8..(i+1)*8].try_into().unwrap())
    });
    limbs.reverse();
    U512::from(limbs)
}


pub fn rsaep(key: &PublicKey, m: U512) -> Result<U512, &'static str> {
   let n = key.n;
   let e = key.e;
   if m >= n {
        return Err("Message representative out of range");
   }
    let c = pow_mod(m, e, n);
   Ok(c)
}


#[allow(non_snake_case)]
pub fn rsadp(key: &PrivateKey, c: U512) -> Result<U512, &'static str> {
    let n: U512 = match key {
        PrivateKey::Pair{n, d: _} => *n,
        PrivateKey::Quintuple{p, q, dp: _, dq: _, q_inv: _, ref rdt} => {
            *p * *q * rdt.iter().fold(U512::ONE, |acc, x| acc * x.1)
        }
    };
    if c >= n {
        return Err("Ciphertext representative out of range");
    }
     
    match key {
        PrivateKey::Pair{n, d} => {
            Ok(pow_mod(c, *d, *n))
        },
        PrivateKey::Quintuple{ p, q, dp, dq, q_inv, ref rdt } => {
            let u = rdt.len();
            let m_1 = pow_mod(c, *dp, *p);
            let m_2 = pow_mod(c, *dq, *q);

            let mut M: Vec<U512> = Vec::with_capacity(u);
            if !rdt.is_empty() {
                for i in 0..u {
                    M[i] = pow_mod(c, rdt[i].1, rdt[i].0);
                } 
            }
            
            let h = m_1.sub_mod(&m_2, &p.to_nz().unwrap()).mul_mod(&q_inv, &p.to_nz().unwrap());
            let mut m = m_2 + q * h; 

            if !rdt.is_empty() {
                let mut R = *p;
                for i in 0..u {
                    if i == 0 {
                        R = R * *q;
                    } else {
                        R = R * rdt[i-1].0;
                    }
                    let h = M[i].sub_mod(&m, &rdt[i].0.to_nz().unwrap()).mul_mod(&rdt[i].2, &rdt[i].0.to_nz().unwrap());
                    m = m + R * h;
                }
            }
            Ok(m)
        },
    }
}


#[allow(non_snake_case)]
pub fn rsasp1(key: &PrivateKey, m: U512) -> Result<U512, &'static str> {
    let n: U512 = match key {
        PrivateKey::Pair{n, d: _} => *n,
        PrivateKey::Quintuple{p, q, dp: _, dq: _, q_inv: _, ref rdt} => {
            *p + *q + rdt.iter().fold(U512::ZERO, |acc, x| acc + x.1)
        }
    };
    if m >= n {
        return Err("Message representative out of range");
    }
     
    match key {
        PrivateKey::Pair{n, d} => {
            Ok(pow_mod(m, *d, *n))
        },
        PrivateKey::Quintuple{ p, q, dp, dq, q_inv, ref rdt } => {
            let u = rdt.len();
            let s_1 = pow_mod(m, *dp, *p);
            let s_2 = pow_mod(m, *dq, *q);

            let mut M: Vec<U512> = Vec::with_capacity(u);
            if !rdt.is_empty() {
                for i in 0..u {
                    M[i] = pow_mod(m, rdt[i].1, rdt[i].0);
                } 
            }
            
            let h = s_1.sub_mod(&s_2, &p.to_nz().unwrap()).mul_mod(&q_inv, &p.to_nz().unwrap());
            let mut s = s_2 + q * h; 

            if !rdt.is_empty() {
                let mut R = *p;
                for i in 0..u {
                    if i == 0 {
                        R = R * *q;
                    } else {
                        R = R * rdt[i-1].0;
                    }
                    let h = M[i].sub_mod(&s, &rdt[i].0.to_nz().unwrap()).mul_mod(&rdt[i].2, &rdt[i].0.to_nz().unwrap());
                    s = s + R * h;
                }
            }
            Ok(s)
        },
    }
}


pub fn rsavp1(key: &PublicKey, s: U512) -> Result<U512, &'static str> {
   let n = key.n;
   let e = key.e;
   if s >= n {
        return Err("Signature representative out of range");
   }
    let m = pow_mod(s, e, n);
   Ok(m)
}
