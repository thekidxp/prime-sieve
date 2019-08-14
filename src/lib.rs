pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        2 => 5,
        3 => 7,
        4 => 11,
        5 => 13,
        _ => improved_sieve(n + 1),
    }
    /*let start = 20;
    let mut primes = primes_under(start);

    if primes.len() > n as usize {
        primes[n as usize]
    } else {
        sieve(n, &mut primes, start as u32)
    }*/
}

fn primes_under(n: usize) -> Vec<u32> {
    let mut sieve = vec![true; n / 2];
    let top = (n as f64).sqrt().round() as usize;
    for i in (3..=top).step_by(2) {
        if sieve[i / 2] {
            for val in sieve[i * i / 2..].iter_mut().step_by(i) {
                *val = false;
            }
        }
    }
    let mut return_vec = vec![2u32];
    for (i, is_prime) in sieve.iter().enumerate().skip(1) {
        if *is_prime {
            return_vec.push((2 * i + 1) as u32)
        }
    }
    return_vec
}

fn sieve(n: u32, primes: &mut Vec<u32>, start: u32) -> u32 {
    match primes.get(n as usize) {
        Some(prime) => prime.to_owned(),
        None => {
            for i in (start..).filter(|x| x % 2 != 0) {
                let mut is_prime = true;
                for prime in primes
                    .iter()
                    .filter(|&p| p <= &((i as f64).sqrt().floor() as u32))
                {
                    if i % *prime == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    primes.push(i);
                }
                if primes.len() == (n + 1) as usize {
                    break;
                }
            }
            primes.get(n as usize).unwrap().to_owned()
        }
    }
}

fn improved_sieve(n: u32) -> u32 {
    let float = f64::from(n);
    let upper_limit = (float * (float.ln() + float.ln().ln())).ceil() as usize;
    let smallest_sieve = ((float * (float.ln() + float.ln().ln())).ceil()).sqrt().ceil() as usize;
    let mut sieve = vec![true; upper_limit / 2];
    let mut primes_found = 1;
    for i in (3..=smallest_sieve).step_by(2) {
        if sieve[i / 2] {
            primes_found += 1;
            for is_prime in sieve[i * i / 2..].iter_mut().step_by(i) {
                *is_prime = false;
            }
        }
    }

    let mut index = (smallest_sieve + 1) / 2;
    for (i, is_prime) in sieve[index..].iter().enumerate() {
        if *is_prime {
            primes_found += 1;
            if primes_found == n {
                index += i;
                break;
            }
        }
    }
    2 * index as u32 + 1
}
