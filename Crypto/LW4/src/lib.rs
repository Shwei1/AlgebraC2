pub mod utils;
#[allow(unused_imports)]
use crypto_bigint::U512;

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    #[test]
    fn test_miller_rabin_1() {
        let inp = U512::from(1u64);
        let res = miller_rabin_test(inp);
        assert_eq!(res, TestResult::NonPrime);
    }

    #[test]
    fn zero() {
        let inp = U512::from(0u64);
        let res = miller_rabin_test(inp);
        assert_eq!(res, TestResult::NonPrime);
    }


    #[test]
    fn composite_28() {
        let inp = U512::from(28u64);
        let res = miller_rabin_test(inp);
        assert_eq!(res, TestResult::NonPrime);
    }

    #[test]
    fn prime_37() {
        let inp = U512::from(37u64);
        let res = miller_rabin_test(inp);
        assert_eq!(res, TestResult::MaybePrime(U512::ZERO));
    }

    #[test]
    fn carmichael_1729() {
        let inp = U512::from(1729u64);
        let res = miller_rabin_test_stats(inp);
        println!("P(1729 is prime) = {}", res);
        assert!(res < 0.25);
    }


    #[test]
    fn carmichael_561() {
        let inp = U512::from(561u64);
        let res = miller_rabin_test_stats(inp);
        println!("P(561 is prime) = {}", res);
        assert!(res < 0.25);
    }


    #[test]
    fn carmichael_1105() {
        let inp = U512::from(1105u64);
        let res = miller_rabin_test_stats(inp);
        println!("P(1105 is prime) = {}", res);
        assert!(res < 0.25);
    }

    #[test]
    fn carmichael_8911() {
        let inp = U512::from(8911u64);
        let res = miller_rabin_test_stats(inp);
        println!("P(8911 is prime) = {}", res);
        assert!(res < 0.25);
    }

    #[test]
    fn test_miller_rabin_prime_1() {
        let inp = U512::from(7919u64);
        let res = miller_rabin_test_stats(inp);
        assert_eq!(res, 1f64);
    }

    #[test]
    fn mersenne_127() {
        let u512_2 = U512::ONE + U512::ONE;
        let inp = u512_2.saturating_pow(&U512::from(127u64))
            .saturating_sub(&U512::ONE);
        let res = miller_rabin_test_stats(inp);
        assert_eq!(res, 1f64);
    }


}
