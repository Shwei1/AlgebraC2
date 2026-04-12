mod sha256;

use sha256::sha256_str;


fn main() {
    let s1 = "";
    let s2 = "abc";
    let s3 = "de188941a3375d3a8a061e67576e926d";
    let s4 = "привіт";

    let s = [s1, s2, s3, s4];

    let a1 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let a2 = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let a3 = "57e918cfef3bd4ecd82e1e01771a60efa713df3d3281f61c785b7f7920e853b7";
    let a4 = "5e7d04a7d8a1d9c665fc624610114b9ec466d23b154148dd74250050fb9e7f19";

    let a = [a1, a2, a3, a4];

    for i in 0..4 {
        let ans = sha256_str(s[i]);
        println!("Hash of \"{}\" is {}.", s[i], ans);
        if ans == a[i] {
            println!("Test {} passed!", i+1);
        } else {
            println!("Test {} failed! Expected {}!", i+1, a[i]);
        }
    }
}
