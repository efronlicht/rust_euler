use super::primes;



#[test]
fn test_primes_under() {
    assert_eq!(vec![2, 3, 5, 7, 11], primes::primes_under(12));
}

#[test]
fn test_factors() {
    let p = primes::primes_under(30);
    let want = Some(hashmap! {2 => 3, 3 => 1});
    assert_eq!(want, primes::factors(8*3, &p));
}

#[test]
fn test_gcd() {
    let p = primes::primes_under(30);
    assert_eq!(primes::gcd(8, 24, &p).unwrap(), 8);
}


#[test]
fn test_lcm() {
    let p = primes::primes_under(30);
    assert_eq!(primes::lcm(3, 5, &p).unwrap(), 15);
}