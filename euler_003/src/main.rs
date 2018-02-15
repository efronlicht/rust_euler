

/*
primes = primes_under(The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/
fn main() {
    let n = 600851475143;
    let primes = primes_under(30000); //
    let facts = factors(n, primes).unwrap();
    println!("{}", facts[facts.len()-1]);

    }

fn primes_under(max: u64) -> Vec<u64> {
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

fn factors(n: u64, primes: Vec<u64>) -> Option<Vec<u64>> {

    let mut factors = Vec::<u64>::new();
    let mut n = n;

    for p in &primes {
        while n % p == 0 {
            factors.push(*p);
            n /= p;
        }
        if p > &n {
            return Some(factors) //we know the prime factorization for sure
        }
    };
    None // n is larger than our largest known prime. it could be prime, but it could also be a sufficiently large composite number
}