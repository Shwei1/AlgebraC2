#![allow(non_snake_case)]
pub mod sha256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "";
        let ans = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(ans, sha256::sha256_str(inp));
    }

    #[test]
    fn test_2() {
        let inp = "de188941a3375d3a8a061e67576e926d";
        let ans = "57e918cfef3bd4ecd82e1e01771a60efa713df3d3281f61c785b7f7920e853b7";
        assert_eq!(ans, sha256::sha256_str(inp));
    }

    #[test]
    fn test_3() {
        let inp = "abc";
        let ans = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert_eq!(ans, sha256::sha256_str(inp));
    }

    #[test]
    fn test_4() {
        let inp = "привіт";
        let ans = "5e7d04a7d8a1d9c665fc624610114b9ec466d23b154148dd74250050fb9e7f19";
        assert_eq!(ans, sha256::sha256_str(inp));
    }
    
    #[test]
    fn test_5() {
        use std::fs;
        let inp = fs::read_to_string("./resources/a.txt").unwrap();
        let inp = inp.trim();
        let ans = "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0";
        assert_eq!(ans, sha256::sha256_str(&inp));
    }
}
