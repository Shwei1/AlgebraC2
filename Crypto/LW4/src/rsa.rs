use crypto_bigint::U512;
use crate::utils::pow_mod;

#[derive(Debug)]
pub struct PublicKey {
    pub n: U512,
    pub e: U512,
}

#[derive(Debug)]
pub enum PrivateKey {
    Pair { n: U512, d: U512},
    #[allow(non_snake_case)]
    Quintuple { p: U512, q: U512, dP: U512, dQ: U512, qInv: U512, rdt: Vec<(U512, U512, U512)>},
}

#[allow(non_snake_case)]
pub fn RSAEP(key: &PublicKey, m: U512) -> Result<U512, &'static str> {
   let n = key.n;
   let e = key.e;
   if m >= n {
        return Err("Message representative out of range");
   }
    let c = pow_mod(m, e, n);
   Ok(c)
}


#[allow(non_snake_case)]
pub fn I2OSP(x: U512, _xLen: usize) -> String {
    todo!()

}


#[allow(non_snake_case)]
pub fn OS2IP(x: &str) -> U512 {
    todo!()
}


#[allow(non_snake_case)]
pub fn RSADP(K: &PrivateKey, c: U512) -> Result<U512, &'static str> {
    let n: U512 = match K {
        PrivateKey::Pair{n, d: _} => *n,
        PrivateKey::Quintuple{p, q, dP: _, dQ: _, qInv: _, ref rdt} => {
            *p * *q * rdt.iter().fold(U512::ONE, |acc, x| acc * x.1)
        }
    };
    if c >= n {
        return Err("Ciphertext representative out of range");
    }
     
    match K {
        PrivateKey::Pair{n, d} => {
            Ok(pow_mod(c, *d, *n))
        },
        PrivateKey::Quintuple{ p, q, dP, dQ, qInv, ref rdt } => {
            let u = rdt.len();
            let m_1 = pow_mod(c, *dP, *p);
            let m_2 = pow_mod(c, *dQ, *q);

            let mut M: Vec<U512> = Vec::with_capacity(u);
            if !rdt.is_empty() {
                for i in 0..u {
                    M[i] = pow_mod(c, rdt[i].1, rdt[i].0);
                } 
            }
            
            let h = m_1.sub_mod(&m_2, &p.to_nz().unwrap()).mul_mod(&qInv, &p.to_nz().unwrap());
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
pub fn RSASP1(K: &PrivateKey, m: U512) -> Result<U512, &'static str> {
    let n: U512 = match K {
        PrivateKey::Pair{n, d: _} => *n,
        PrivateKey::Quintuple{p, q, dP: _, dQ: _, qInv: _, ref rdt} => {
            *p + *q + rdt.iter().fold(U512::ZERO, |acc, x| acc + x.1)
        }
    };
    if m >= n {
        return Err("Message representative out of range");
    }
     
    match K {
        PrivateKey::Pair{n, d} => {
            Ok(pow_mod(m, *d, *n))
        },
        PrivateKey::Quintuple{ p, q, dP, dQ, qInv, ref rdt } => {
            let u = rdt.len();
            let s_1 = pow_mod(m, *dP, *p);
            let s_2 = pow_mod(m, *dQ, *q);

            let mut M: Vec<U512> = Vec::with_capacity(u);
            if !rdt.is_empty() {
                for i in 0..u {
                    M[i] = pow_mod(m, rdt[i].1, rdt[i].0);
                } 
            }
            
            let h = s_1.sub_mod(&s_2, &p.to_nz().unwrap()).mul_mod(&qInv, &p.to_nz().unwrap());
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


#[allow(non_snake_case)]
pub fn RSAVP1(key: &PublicKey, s: U512) -> Result<U512, &'static str> {
   let n = key.n;
   let e = key.e;
   if s >= n {
        return Err("Signature representative out of range");
   }
    let m = pow_mod(s, e, n);
   Ok(m)
}
