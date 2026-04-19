use rand::RngCore;
use rand::Rng;
use rand::thread_rng;
use LW2::aes;

fn main() {
    println!("Changing the amount of rounds for AES-128");
    for i in 20..24 {
        println!("Custom amount of rounds {}:", i);
        println!("1) Modifying one bit in the text:");
        println!("\tCalculating...");
        let mut popcount_total = 0usize;
        for _ in 0..100_000 {
            let mut rand_bytes = [0u8; 16];
            let key = [1u8; 16];
            thread_rng().fill_bytes(&mut rand_bytes);

            let original = aes::aes_128_custom(&rand_bytes, &key, i);

            let byte_idx = thread_rng().gen_range(0..16);
            let bit_idx = thread_rng().gen_range(0..8);

            rand_bytes[byte_idx] ^= 1 << bit_idx;

            let changed = aes::aes_128_custom(&rand_bytes, &key, i);

            let popcount: usize = original.iter()
                .zip(changed.iter())
                .map(|(a, b)| (a ^ b).count_ones() as usize)
                .sum();
            popcount_total += popcount;
        }
        println!("\tOn average {} bits differ", popcount_total / 100_000);
        println!("2) Modifying one bit in the key:");
        println!("\tCalculating...");
        let mut popcount_total = 0usize;
        for _ in 0..100_000 {
            let text = [1u8; 16];
            let mut rand_key = [0u8; 16];
            thread_rng().fill_bytes(&mut rand_key);

            let original = aes::aes_128_custom(&text, &rand_key, i);

            let byte_idx = thread_rng().gen_range(0..16);
            let bit_idx = thread_rng().gen_range(0..8);

            rand_key[byte_idx] ^= 1 << bit_idx;

            let changed = aes::aes_128_custom(&text, &rand_key, i);

            let popcount: usize = original.iter()
                .zip(changed.iter())
                .map(|(a, b)| (a ^ b).count_ones() as usize)
                .sum();
            popcount_total += popcount;
        }
        println!("\tOn average {} bits differ", popcount_total / 100_000);
        }  
    for i in 5..10 {
        println!("Custom amount of rounds {}:", i);
        println!("1) Modifying one bit in the text:");
        println!("\tCalculating...");
        let mut popcount_total = 0usize;
        for _ in 0..100_000 {
            let mut rand_bytes = [0u8; 16];
            let key = [1u8; 16];
            thread_rng().fill_bytes(&mut rand_bytes);

            let original = aes::aes_128_custom(&rand_bytes, &key, i);

            let byte_idx = thread_rng().gen_range(0..16);
            let bit_idx = thread_rng().gen_range(0..8);

            rand_bytes[byte_idx] ^= 1 << bit_idx;

            let changed = aes::aes_128_custom(&rand_bytes, &key, i);

            let popcount: usize = original.iter()
                .zip(changed.iter())
                .map(|(a, b)| (a ^ b).count_ones() as usize)
                .sum();
            popcount_total += popcount;
        }
        println!("\tOn average {} bits differ", popcount_total / 100_000);
        println!("2) Modifying one bit in the key:");
        println!("\tCalculating...");
        let mut popcount_total = 0usize;
        for _ in 0..100_000 {
            let text = [1u8; 16];
            let mut rand_key = [0u8; 16];
            thread_rng().fill_bytes(&mut rand_key);

            let original = aes::aes_128_custom(&text, &rand_key, i);

            let byte_idx = thread_rng().gen_range(0..16);
            let bit_idx = thread_rng().gen_range(0..8);

            rand_key[byte_idx] ^= 1 << bit_idx;

            let changed = aes::aes_128_custom(&text, &rand_key, i);

            let popcount: usize = original.iter()
                .zip(changed.iter())
                .map(|(a, b)| (a ^ b).count_ones() as usize)
                .sum();
            popcount_total += popcount;
        }
        println!("\tOn average {} bits differ", popcount_total / 100_000);
        }  
}


