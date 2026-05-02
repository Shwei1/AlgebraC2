use lw4::rsa::*;
use oaep::*;
use crypto_bigint::U512;

#[test]
fn test_oaep_roundtrip_1() {
    let n = U512::from_be_hex("b968c6174730b15c80f4d6227112acfb7a29c630c95c43d4bdd29cb18489ff203b97b779c6d1ec9bef2c7bce55f41189bb04d153096cc0248a13a03a2ce7c119");
    let e = U512::from(65537u32);
    let d = U512::from_be_hex("84355deb836c094cb8f7dd29ba0f6321ff3e82808f5527592ff25aa18aad0c2a2cd0a4abacbefe3c9ed1b9d8f1e9f753681e21e8d88194eba728e3741bd92a01");

    let public_key = PublicKey { n, e };
    let private_key = PrivateKey::Pair{ n, d };

    let m = b"hello"; 

    let c = rsaes_oaep_encrypt(&public_key, m, None).unwrap();
    let m2 = rsaes_oaep_decrypt(&private_key, &c, None).unwrap();


    assert_eq!(m, m2.as_slice());
}


#[test]
fn test_oaep_roundtrip_2() {
    let n = U512::from_be_hex("a5bd3d6a43efabef0388db6e38df8139e909a90d7cd3a1eecf8e8380fc4388a64309db1820bac9e0362f191f39c6e35a09754b97432df9e4c47fe9aedc800191");
    let e = U512::from(65537u64);

    let p = U512::from_be_hex(&format!("{:0>128}", "00d61761a0660ce661599541b4ec372acf11e4b8089dcd20d1e4f41a6d43d876f7"));
    let q = U512::from_be_hex(&format!("{:0>128}", "00c62ed219b0c0b89f9aaf4306ad35f1abdbb4157676183f6849965ad04f1401b7"));
    let dp = U512::from_be_hex(&format!("{:0>128}", "5639a6852e74c598b1b6ce76eaa83162a2a7468399e292f4564d0613b9be85f3"));
    let dq = U512::from_be_hex(&format!("{:0>128}", "497c61018fadd687b006ded6a1f187bc4f0a368f27791441ea4f6b3b24a7697f"));
    let q_inv = U512::from_be_hex(&format!("{:0>128}", "07e2b9f480d9c3b6275376e9719633d38c1e157a0f7cc46b124978cadd4e8a1c"));

    let public_key = PublicKey { n, e };
    let private_key = PrivateKey::Quintuple { p, q, dp, dq, q_inv, rdt: Vec::new()};

    let m = b"howareyou"; 

    let c = rsaes_oaep_encrypt(&public_key, m, None).unwrap();
    let m2 = rsaes_oaep_decrypt(&private_key, &c, None).unwrap();


    assert_eq!(m, m2.as_slice());
}


#[test]
#[should_panic]
fn test_oaep_roundtrip_3() {
    let n = U512::from_be_hex("a5bd3d6a43efabef0388db6e38df8139e909a90d7cd3a1eecf8e8380fc4388a64309db1820bac9e0362f191f39c6e35a09754b97432df9e4c47fe9aedc800191");
    let e = U512::from(65537u64);

    let p = U512::from_be_hex(&format!("{:0>128}", "00d61761a0660ce661599541b4ec372acf11e4b8089dcd20d1e4f41a6d43d876f7"));
    let q = U512::from_be_hex(&format!("{:0>128}", "00c62ed219b0c0b89f9aaf4306ad35f1abdbb4157676183f6849965ad04f1401b7"));
    let dp = U512::from_be_hex(&format!("{:0>128}", "5639a6852e74c598b1b6ce76eaa83162a2a7468399e292f4564d0613b9be85f3"));
    let dq = U512::from_be_hex(&format!("{:0>128}", "497c61018fadd687b006ded6a1f187bc4f0a368f27791441ea4f6b3b24a7697f"));
    let q_inv = U512::from_be_hex(&format!("{:0>128}", "07e2b9f480d9c3b6275376e9719633d38c1e157a0f7cc46b124978cadd4e8a1c"));

    let public_key = PublicKey { n, e };
    let private_key = PrivateKey::Quintuple { p, q, dp, dq, q_inv, rdt: Vec::new()};

    let m = b"omnishominesquisesestudentpraestare"; 

    let c = rsaes_oaep_encrypt(&public_key, m, None).unwrap();
    let m2 = rsaes_oaep_decrypt(&private_key, &c, None).unwrap();


    assert_eq!(m, m2.as_slice());
}
