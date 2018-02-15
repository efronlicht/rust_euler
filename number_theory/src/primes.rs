use std::collections;
type Counter = collections::HashMap<u64, u32>;
type Primes = Vec<u64>;

pub fn union(a: &Counter, b: &Counter) -> Counter {
    let mut union = a.clone();
    for (k, v) in b {
         if !union.contains_key(k) {
                union.insert(*k, *v);
            } else {
                let m = union[k];
                union.insert(*k, m.max(*v));
        }
    }
    union
}


pub fn intersection(a: &Counter, b: &Counter) -> Counter {
    let mut intersection = collections::HashMap::new();
    for (k, v) in a {
        if b.contains_key(k) {
            let min = *v.min(&b[k]);
            intersection.insert(*k, min);
        }
    }
    intersection
}
///gcd is the greatest common divisor.  that is, the largest d such that ad == m bd == n for some positive integers a, b
pub fn gcd(m: u64, n: u64, primes: &Primes) -> Option<u64> {
    if m == 0 || n == 0 {return None}
    let intersection = intersection(&factors(n, primes)?, &factors(m, primes)?);
    let mut gcd = 1;
    for (p, exp) in intersection {
        gcd *= p.pow(exp);
    }
    Some(gcd)
}

//lcm is the least common multiple; that is, the smallest q such that a*m == q, b*m == q for some positive integers a, b
pub fn lcm(m: u64, n: u64, primes: &Primes) -> Option<u64> {
    if m == 0 || n == 0 {
        return Some(0)
    }
    let union = union(&factors(n, primes)?, &factors(m, primes)?);
    let mut lcm = 1;
    for (p, exp) in union.iter() {
        lcm *= p.pow(*exp);
    }
    Some(lcm)
}


pub fn primes_under(max: u64) -> Primes {
    let mut primes = Vec::new();
    if max < 2 {
        return primes    
    }
    primes.push(2);
    let mut n = 3;


    while n < max{
        let mut isprime = true;
        for p in &primes {
            if n % p == 0 {
                isprime = false;
                break 
            }
        }
        if isprime {
            primes.push(n);
        }

        n +=2
    }
    primes
}



pub fn factors(n: u64, primes: &Primes) -> Option<Counter>{

    let mut factors = collections::HashMap::new();
    let mut n = n;

    for p in primes {
        while n % p == 0 {
            *factors.entry(n).or_insert(0) += 1;
            n /= p;
        }
        if p > &n {
            return Some(factors) //we know the prime factorization for sure
        }
    };
    None // n is larger than our largest known prime. it could be prime, but it could also be a sufficiently large composite number
}

