use crate::utils::*;

pub struct PublicParams {
    p: u32,
    g: u32,
}


impl PublicParams {
    pub fn new(p: u32, g: u32) -> Result<Self, &'static str> {
        if order(g, p)? != euler_totient(p) {
            return Err("g is not a generator of (Z/Z_p)*");
        }
        Ok(PublicParams { p, g })
    }
}

pub fn generate_private_key(params: &PublicParams) -> u32 {
    rand::random_range(2..params.p-2)
}

pub fn generate_public_key(params: &PublicParams, private_key: u32) -> u32 {
    power_modulo(params.g, private_key, params.p)
}

pub fn compute_shared_secret(params: &PublicParams, my_private_key: u32, other_public_key: u32) -> u32 {
    power_modulo(other_public_key, my_private_key, params.p)
}





