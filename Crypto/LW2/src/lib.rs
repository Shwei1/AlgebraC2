#![allow(non_snake_case)]

pub mod aes;
mod gf256;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_128_1() {
        let inp: [u8; 16] = hex::decode("6bc1bee22e409f96e93d7e117393172a").unwrap().try_into().unwrap();
        let key: [u8; 16] = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("3ad77bb40d7a3660a89ecaf32466ef97").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_128(&inp, &key), ans);
    }

    #[test]
    fn test_128_2() {
        let inp: [u8; 16] = hex::decode("ae2d8a571e03ac9c9eb76fac45af8e51").unwrap().try_into().unwrap();
        let key: [u8; 16] = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("f5d3d58503b9699de785895a96fdbaaf").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_128(&inp, &key), ans);
    }

    #[test]
    fn test_192_1() {
        let inp: [u8; 16] = hex::decode("1b077a6af4b7f98229de786d7516b639").unwrap().try_into().unwrap();
        let key: [u8; 24] = hex::decode("000000000000000000000000000000000000000000000000").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("275cfc0413d8ccb70513c3859b1d0f72").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_192(&inp, &key), ans);
    }


    #[test]
    fn test_192_2() {
        let inp: [u8; 16] = hex::decode("9c2d8842e5f48f57648205d39a239af1").unwrap().try_into().unwrap();
        let key: [u8; 24] = hex::decode("000000000000000000000000000000000000000000000000").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("c9b8135ff1b5adc413dfd053b21bd96d").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_192(&inp, &key), ans);
    }

    #[test]
    fn test_192_3() {
        let inp: [u8; 16] = hex::decode("00000000000000000000000000000000").unwrap().try_into().unwrap();
        let key: [u8; 24] = hex::decode("e9f065d7c13573587f7875357dfbb16c53489f6a4bd0f7cd").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("0956259c9cd5cfd0181cca53380cde06").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_192(&inp, &key), ans);
    }

    #[test]
    fn test_192_4() {
        let inp: [u8; 16] = hex::decode("00000000000000000000000000000000").unwrap().try_into().unwrap();
        let key: [u8; 24] = hex::decode("d2926527e0aa9f37b45e2ec2ade5853ef807576104c7ace3").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("dd619e1cf204446112e0af2b9afa8f8c").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_192(&inp, &key), ans);
    }

    #[test]
    fn test_256_1() {
        let inp: [u8; 16] = hex::decode("00000000000000000000000000000000").unwrap().try_into().unwrap();
        let key: [u8; 32] = hex::decode("c47b0294dbbbee0fec4757f22ffeee3587ca4730c3d33b691df38bab076bc558").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("46f2fb342d6f0ab477476fc501242c5f").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_256(&inp, &key), ans);
    }

    #[test]
    fn test_256_2() {
        let inp: [u8; 16] = hex::decode("00000000000000000000000000000000").unwrap().try_into().unwrap();
        let key: [u8; 32] = hex::decode("797f8b3d176dac5b7e34a2d539c4ef367a16f8635f6264737591c5c07bf57a3e").unwrap().try_into().unwrap();
        let ans: [u8; 16] = hex::decode("a74289fe73a4c123ca189ea1e1b49ad5").unwrap().try_into().unwrap();
        assert_eq!(aes::aes_256(&inp, &key), ans);
    }
}



