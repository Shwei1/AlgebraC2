use crate::rsa::*;
use crate::utils::*;
use lw3::sha256::insecure::sha1;
use rand::RngExt;


pub fn rsaes_oaep_encrypt(key: &PublicKey, m: &[u8], l: Option<&[u8]>) -> Result<Vec<u8>, &'static str> {
    let h_len = 20;
    let l = match l {
        Some(s) => s,
        None => &[],
    };

    if l.len() > ((2u64 << 60) - 1) as usize {
        return Err("Label too long");
    }
    if m.len() > (64 - 2 * h_len - 2) as usize {
        return Err("Message too long");
    }

    let l_hash = sha1(l);

    let mut ps = Vec::with_capacity(64 - 2 * h_len - 2 - m.len());
    ps.resize(64 - 2 * h_len - 2 - m.len(), 0u8);

    let db = [&l_hash, ps.as_slice(), &[0x01u8], m].concat();
    
    let mut seed = [0u8; 20];
    rand::rng().fill(&mut seed);

    let db_mask = mgf1(&seed, (64-h_len-1) as u64).unwrap();
    assert_eq!(db.len(), db_mask.len());

    let masked_db: Vec<u8> = db.into_iter()
        .zip(db_mask.into_iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let seed_mask = mgf1(&masked_db, h_len as u64).unwrap();

    let masked_seed: Vec<u8> = seed.into_iter()
        .zip(seed_mask.into_iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let em = [&[0u8], masked_seed.as_slice(), masked_db.as_slice()].concat();

    let m = os2ip(&em);
    let c = rsaep(&key, m).unwrap();

    Ok(i2osp(c, 64).unwrap().to_vec())
}


pub fn rsaes_oaep_decrypt(key: &PrivateKey, c: &[u8], l: Option<&[u8]>) -> Result<Vec<u8>, &'static str> {
    let h_len = 20;
    let l = match l {
        Some(s) => s,
        None => &[],
    };
    
    if c.len() != 64 {
        return Err("Decryption error");
    }
    if l.len() > ((2u64 << 60) - 1) as usize {
        return Err("Decryption error");
    }

    let c = os2ip(c);
    let m = rsadp(&key, c)?;

    let em = i2osp(m, 64).unwrap();

    let l_hash = sha1(l);
    

    let y = em[0];

    if y != 0x00 {
    return Err("Decryption error");
}

    let (masked_seed, masked_db) = em[1..].split_at(h_len);

    let seed_mask = mgf1(masked_db, h_len as u64).unwrap();

    let seed: Vec<u8> = masked_seed.into_iter()
        .zip(seed_mask.into_iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let db_mask = mgf1(&seed, (64-h_len-1) as u64).unwrap();

    let db: Vec<u8> = masked_db.into_iter()
        .zip(db_mask.into_iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    let (l_hash_prime, db_remnant) = db.split_at(h_len);
    if l_hash_prime != l_hash {
        return Err("Decryption error");
    }
    let idx_one = db_remnant.into_iter().position(|b| *b == 0x01u8);
    let idx_one = if let Some(i) = idx_one {
        i
    } else {
        return Err("Decryption error");
    };

    if db_remnant[..idx_one].iter().any(|&b| b != 0u8) {
        return Err("Decryption error");
    }

    let m = &db_remnant[idx_one+1usize..];

    Ok(m.to_vec())
}
