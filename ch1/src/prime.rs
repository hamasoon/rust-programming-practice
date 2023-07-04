fn is_prime(n: u32) -> bool {
    for i in 2..n/2 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn get_primes(primes: &mut[bool; 1000]) {
    primes[0] = false;
    primes[1] = false;

    for i in 2..1000 {
        primes[i] = is_prime(i as u32);
    }
}

fn main() {
    let mut primes = [true; 1000];

    get_primes(&mut primes);

    for i in 0..1000 {
        if primes[i] {
            println!("{}", i);
        }
    }
}

