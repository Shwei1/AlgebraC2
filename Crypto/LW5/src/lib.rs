pub mod utils;
pub mod diffie_hellman;


#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::diffie_hellman::*;

    #[test]
    fn test_find_generator_1() {
        let n = 433;
        let a = 7;
        let res = find_generator(n).unwrap();
        assert_eq!(order(res, n), order(a, n));
    }


    #[test]
    fn test_find_generator_2() {
        let n = 2;
        assert_eq!(find_generator(n).unwrap(), 1);
    }


    #[test]
    fn find_order_1() {
        let n = 233;
        let x = 23;
        assert_eq!(order(x, n).unwrap(), 29);
    }

    #[test]
    #[should_panic]
    fn find_generator_bad() {
        let n = 25;
        let x = 5;
        assert_eq!(order(x, n).unwrap(), 5);
    }


    #[test]
    fn test_discrete_log_1() {
        let n = 433;
        let a = 7;
        let x = 166;

        assert_eq!(discrete_log(x, a, n).unwrap(), 47);
    }

    
    #[test]
    fn test_diffie_hellman_1() {
        let pub_params = PublicParams::new(23, 5).unwrap();

        let alice_private = 4;
        let alice_public = generate_public_key(&pub_params, alice_private);

        let bob_private = 3;
        let bob_public = generate_public_key(&pub_params, bob_private);

        let secret_msg_from_bob = compute_shared_secret(&pub_params, alice_private, bob_public);
        let secret_msg_from_alice = compute_shared_secret(&pub_params, bob_private, alice_public);


        assert_eq!(secret_msg_from_bob, secret_msg_from_alice);
    }


    #[test]
    fn test_diffie_hellman_big() {
        let p = 1674260501;
        let g = find_generator(1674260501).unwrap();

        println!("Public parameters: {p}");
        println!("Generator: {g}");

        let pub_params = PublicParams::new(p, g).unwrap();

        let alice_private = generate_private_key(&pub_params);
        let alice_public = generate_public_key(&pub_params, alice_private);

        println!("Alice's private key: {alice_private}");
        println!("Alice's public key: {alice_public}");

        let bob_private = generate_private_key(&pub_params);
        let bob_public = generate_public_key(&pub_params, bob_private);

        println!("Bob's private key: {bob_private}");
        println!("Bob's public key: {bob_public}");

        let secret_msg_from_bob = compute_shared_secret(&pub_params, alice_private, bob_public);
        let secret_msg_from_alice = compute_shared_secret(&pub_params, bob_private, alice_public);

        println!("Alice received message: {secret_msg_from_bob}");
        println!("Bob received message: {secret_msg_from_alice}");


        assert_eq!(secret_msg_from_bob, secret_msg_from_alice);
    }


    #[test]
    #[should_panic]
    fn diffie_hellman_test_bad_1() {
        let p = 7;
        let g = 2;

        println!("Public parameters: {p}");
        println!("Generator: {g}");

        let pub_params = PublicParams::new(p, g).unwrap();

        let alice_private = generate_private_key(&pub_params);
        let alice_public = generate_public_key(&pub_params, alice_private);

        println!("Alice's private key: {alice_private}");
        println!("Alice's public key: {alice_public}");

        let bob_private = generate_private_key(&pub_params);
        let bob_public = generate_public_key(&pub_params, bob_private);

        println!("Bob's private key: {bob_private}");
        println!("Bob's public key: {bob_public}");

        let secret_msg_from_bob = compute_shared_secret(&pub_params, alice_private, bob_public);
        let secret_msg_from_alice = compute_shared_secret(&pub_params, bob_private, alice_public);

        println!("Alice received message: {secret_msg_from_bob}");
        println!("Bob received message: {secret_msg_from_alice}");


        assert_eq!(secret_msg_from_bob, secret_msg_from_alice);

    }

}
