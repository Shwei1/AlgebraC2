mod sha256;

use std::io;
use sha256::sha256_str;

fn main() 
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    println!("{}", sha256_str(s));
}
