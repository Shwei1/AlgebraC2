use std::collections::HashMap;
use num::integer::Integer;


pub fn find_generator(n: u32) -> Result<u32, &'static str> {
    if n == 0 || n == 1 {
        return Err("No such group Z/{e}");
    }
    if n == 2 {
        return Ok(1);
    }
    let phi = euler_totient(n);
    let factorization = factorize_incompletely(phi);
    let mut a;
    'outer: loop {
        a = rand::random_range(2..n);
        for i in 0..factorization.len() {
            let b = power_modulo(a, phi/factorization[i], n);
            if b == 1 {
                continue 'outer;
            }
        }
        break;
    }
    Ok(a)
}


pub fn factorize_incompletely(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut m = n;
    let mut d = 2;
    while d * d <= m {
        if m % d == 0 {
            result.push(d);
            while m % d == 0 {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        result.push(m);
    }
    result
}


pub fn factorize(mut n: u32) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    let mut d = 2;
    while d * d <= n {
        while (n % d) == 0 {
            result.entry(d)
                .and_modify(|val| *val += 1)
                .or_insert(1);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        result.insert(n, 1);
    }
    result
}


///
/// x - the element
/// a - the generator
/// n - the order of Z_n^*
/// returns log_a(x)
///
pub fn discrete_log(x: u32, a: u32, n: u32) -> Result<u32, &'static str> {
    let ord = order(a, n)?;
    let m = ord.isqrt() + 1;

    let mut baby_steps = HashMap::new();
    for i in 0..m {
        baby_steps.insert(power_modulo(a, i, n), i);
    }
    let a_inv_m = power_modulo(a, ord - m % ord, n);

    let mut giant = x;
    for j in 0..m {
        if let Some(&i) = baby_steps.get(&giant) {
            return Ok(i + m * j);
        }
        giant = (giant as u64 * a_inv_m as u64 % n as u64) as u32;
    }
    unreachable!()
}


pub(super) fn power_modulo(mut base: u32, mut exp: u32, modulus: u32) -> u32 {
    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 { res = (res as u64 * base as u64 % modulus as u64) as u32; }
        base = (base as u64 * base as u64 % modulus as u64) as u32;
        exp /= 2;
    }
    res
}


pub fn order(x: u32, n: u32) -> Result<u32, &'static str> {
    if x.gcd(&n) != 1 {
        return Err("x is not in (Z/Z_n)*");
    }
    let phi = euler_totient(n);
    let mut t = phi;
    let factors = factorize(phi);
    for (p, _) in &factors {
        while t % p == 0 && power_modulo(x, t / p, n) == 1 {
            t /= p;
        }
    }
    Ok(t)
}


pub fn euler_totient(n: u32) -> u32 {
    let factors = factorize(n);
    let mut result = n;
    for (p, _) in factors {
        result -= result / p;
    }
    result
}


