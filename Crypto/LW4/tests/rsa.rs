use lw4::rsa;
use crypto_bigint::U512;

#[test]
fn test_rsa_primitive_1() {
    let _p = U512::from(61u32);
    let _q = U512::from(53u32);
    let n = U512::from(3233u32); 
    let e = U512::from(17u32);   
    let d = U512::from(413u32);  

    let public_key = rsa::PublicKey { n, e };
    let private_key = rsa::PrivateKey::Pair { n, d };

    let m = U512::from(42u32);

    let c = rsa::rsaep(&public_key, m).expect("encryption failed");
    let m2 = rsa::rsadp(&private_key, c).expect("decryption failed");

    assert_eq!(m, m2);
}


#[test]
fn test_rsa_primitive_quintuple() {
    let p = U512::from(61u32);
    let q = U512::from(53u32);
    let n = U512::from(3233u32);
    let e = U512::from(17u32);

    let dp = U512::from(53u32);   
    let dq = U512::from(49u32);  
    let q_inv = U512::from(38u32); 

    let private_key = rsa::PrivateKey::Quintuple {
        p, q, dp, dq, q_inv,
        rdt: vec![],  
    };

    let public_key = rsa::PublicKey { n, e };

    let m = U512::from(42u32);
    let c = rsa::rsaep(&public_key, m).expect("encryption failed");
    let m2 = rsa::rsadp(&private_key, c).expect("decryption failed");

    assert_eq!(m, m2);
}


#[test]
    fn test_rsaep_out_of_range() {
        let public_key = rsa::PublicKey { n: U512::from(3233u32), e: U512::from(17u32) };
        let m = U512::from(9999u32);  
        assert!(rsa::rsaep(&public_key, m).is_err());
    }

