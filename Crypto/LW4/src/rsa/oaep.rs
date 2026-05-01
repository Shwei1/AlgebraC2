use crate::rsa::*;
use crate::utils::*;
use lw3::sha256::insecure::sha1;

#[allow(non_snake_case)]
pub fn RSAES_OAEP_ENCRYPT(K: PublicKey, M: &str, L: Option<&str>) -> Result<String, &'static str> {
    let L = match L {
        Some(s) => s,
        None => "",
    };


    
}
