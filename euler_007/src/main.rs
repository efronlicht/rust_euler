extern crate number_theory;
use number_theory::primes;
/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/
fn main() {
    let p = primes::Primes::first_n(10001);
    println!("{}", p.max().unwrap())
}
