use crypto_bigint::{U256, U512, U1024, Random, RandomMod, NonZero};
use crate::rsa;
use lw3::sha256::insecure::sha1;

#[derive(Debug)]
pub enum TestResult {
    MaybePrime(U512),
    NonPrime,
}

impl PartialEq for TestResult {
    fn eq(&self, other: &TestResult) -> bool {
        match (self, other) {
            (TestResult::MaybePrime(..), TestResult::MaybePrime(..)) => true,
            (TestResult::NonPrime, TestResult::NonPrime) => true,
            _ => false,
        }
    }
}


pub fn pow_mod(base: U512, mut exp: U512, modulus: U512) -> U512 {
    let mut result = U512::ONE;
    let u512_2: NonZero<U512> = NonZero::new(U512::ONE.saturating_add(&U512::ONE)).unwrap();
    let modulus_nz = NonZero::new(modulus).unwrap();
    let wide_modulus = NonZero::new(U1024::from(&modulus)).unwrap();
    let mut base = base.div_rem(&modulus_nz).1;

    while exp > U512::ZERO {
        if exp.div_rem(&u512_2).1 == U512::ONE {
            let (lo, hi) = result.widening_mul(&base);
            let wide = U1024::from((lo, hi));
            let bytes = wide.div_rem(&wide_modulus).1.to_le_bytes();
            result = U512::from_le_slice(&bytes[..64]);
        }
        exp = exp.wrapping_shr(1);
        let (lo, hi) = base.widening_mul(&base);
        let wide = U1024::from((lo, hi));
        let bytes = wide.div_rem(&wide_modulus).1.to_le_bytes();
        base = U512::from_le_slice(&bytes[..64]);
    }
    result
}


pub fn miller_rabin_test(n: U512) -> TestResult {

    let u512_2: NonZero<U512> = NonZero::new(U512::ONE.saturating_add(&U512::ONE)).unwrap();

    if n == U512::ONE || n == U512::ZERO {
        return TestResult::NonPrime;
    } else if n.to_nz().unwrap() == u512_2 {
        return TestResult::MaybePrime(U512::ONE);
    } else if n.div_rem(&u512_2).1 == U512::ZERO {
        return TestResult::NonPrime;
    }

    let modulus = n.saturating_sub(&U512::ONE);

    let modulus_nz = NonZero::new(modulus).unwrap(); /* n-1 */

    let a = U512::random_mod_vartime(&mut rand::rng(), &modulus_nz); /* a in [0, n-2]*/
    let a = a.saturating_add(&U512::ONE); /* shift: a in [1, n-1] */


    let mut d = n.saturating_sub(&U512::ONE);
 
    d = d.wrapping_div(&u512_2);
    while d.div_rem(&u512_2).1 != U512::ONE {
         
        let ad = pow_mod(a, d, n);

        if ad == modulus {
            return TestResult::MaybePrime(a);
        }
        
        d = d.wrapping_div(&u512_2);
    }
    
    let ad = pow_mod(a, d, n);
    if ad == U512::ONE || ad == modulus {
        return TestResult::MaybePrime(a);
    }

    TestResult::NonPrime
}


pub fn miller_rabin_test_stats(n: U512, k: u32) -> f64 {
    let mut counted_as_prime = 0f64; 
    for _ in 0..k {
        match miller_rabin_test(n) {
            TestResult::MaybePrime(_) => counted_as_prime += 1f64,
            TestResult::NonPrime => {},
        }
    }
    counted_as_prime / k as f64
}


pub fn mgf1(z: &[u8], l: u64) -> Result<Vec<u8>, &'static str> {
    let h_len = 20u64;
    if l > (h_len << 32) {
        return Err("Mask too long");
    }
    let mut t = Vec::new();
    
    let mut i: u64 = 0;
    while t.len() < l as usize {
        let c = rsa::i2osp(U512::from(i), 4).unwrap();
        t.extend_from_slice(&sha1(&[z, &c].concat()));
        i += 1;
    }

    Ok(t[..l as usize].to_vec())
}


fn generate_prime() -> U512 {
    const SMALL_PRIMES: [u64; 12] = [3,5,7,11,13,17,19,23,29,31,37,41];

    'outer: loop {
       let mut n = U256::random();
       n |= U256::ONE.shl(255);
       n |= U256::ONE;

        for p in SMALL_PRIMES {
            if n.rem(&U256::from(p).to_nz().unwrap()) == U256::ZERO {
                continue 'outer;
            }
        }

        let mut limbs = [0u64; 8];

        limbs[..4].copy_from_slice(&n.to_words());

        let n = U512::from_words(limbs);

        if miller_rabin_test_stats(n, 8) > 0.9 {
            return n;
        }
   } 
}


pub fn keygen() -> (rsa::PublicKey, (rsa::PrivateKey, rsa::PrivateKey)) {
    let p = generate_prime();
    let mut q = generate_prime();

    while p == q {
        q = generate_prime();
    }

    let e = U512::from(65537u32);

    let n = p * q;

    let p1 = p - U512::ONE;
    let p2 = q - U512::ONE;

    let gcd = p1.gcd(&p2);
    let lambda = (p1 * p2)/gcd;

    assert!(e.gcd(&lambda) == U512::ONE);

    let d = e.invert_mod(&lambda.to_nz().unwrap()).unwrap();

    let dp = d.rem(&p1.to_nz().unwrap());
    let dq = d.rem(&p2.to_nz().unwrap());
    let q_inv = q.invert_mod(&p.to_nz().unwrap()).unwrap();

    (rsa::PublicKey { n, e },
     (rsa::PrivateKey::Pair { n, d },
      rsa::PrivateKey::Quintuple { p, q, dp, dq, q_inv, rdt: Vec::new()} ))
}
