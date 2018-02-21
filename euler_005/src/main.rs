
extern crate number_theory;
use number_theory::primes;

///2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
fn main() {
    let p = primes::Primes::under(25);
    let union = (2..21)
        .map(|n| primes::factors(n, &p).unwrap())
        .fold(primes::Factors::new(), |a, b| primes::union(&a, &b));
    let lcm: u64 = union.iter().map(|(base, exp)| base.pow(*exp as u32)).product();
    println!("{}", lcm)
}