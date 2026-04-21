use LW3::sha256;
use rand::Rng;

use std::time::Instant;
use std::time::Duration;
use std::collections::HashSet;

struct BenchmarkRecord {
    k: u32,
    avg_time: u128,
}


fn create_bit_prefix(hash: &[u8; 32], k: u32) -> u128 {
    let mut res = 0u128;

    for i in 0..k as usize {
        let byte = hash[i >> 3];
        let bit = (byte >> (7 - (i & 7))) & 1;
        res = (res << 1) | (bit as u128);
    }

    res
}

fn first_k_matches(k: u32) -> Duration {
    let mut rng = rand::thread_rng();

    let mut occurences: HashSet<u128> = HashSet::new();
    
    let start = Instant::now();
    loop {
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);


        let hash = sha256::sha256(&bytes);
        let prefix = create_bit_prefix(&hash, k);
        
        if !occurences.insert(prefix) {
            return start.elapsed();
        }
    }
}


fn main() {
    let mut results: Vec<BenchmarkRecord> = Vec::with_capacity(11);
    
    for k in 5..16 {
        let mut time = 0u128;
        for _ in 0..100 {
            let res = first_k_matches(k);
            time += res.as_nanos();
        }
        results.push(BenchmarkRecord {k: k, avg_time: time / 100});
    }
    
    println!("| k | time (ns)");
    for rec in &results {
        println!("| {} | {} |", rec.k, rec.avg_time);
    }
}
