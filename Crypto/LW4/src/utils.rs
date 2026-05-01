use crypto_bigint::{U512, U1024, RandomMod, NonZero};

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


pub fn miller_rabin_test_stats(n: U512) -> f64 {
    let mut counted_as_prime = 0f64; 
    for _ in 0..100 {
        match miller_rabin_test(n) {
            TestResult::MaybePrime(_) => counted_as_prime += 1f64,
            TestResult::NonPrime => {},
        }
    }
    counted_as_prime / 100f64
}
