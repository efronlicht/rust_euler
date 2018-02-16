use super::primes::*;

#[test]
fn test_primes_under() {
    let p = primes_under(12);
    assert_eq!(vec![2, 3, 5, 7, 11], p);
}

#[test]
fn test_factors() {
    let want = Some(hashmap! {2 => 3, 3 => 1});
    assert_eq!(want, factors(8*3, &primes_under(12)));
}

#[test]
fn test_gcd() { // tests intersection implicitly
    let got = gcd(2*3*5, 2*2*2*3, &primes_under(12)).unwrap();
    assert_eq!(got, 2*3);
}


#[test]
fn test_lcm() {
    let got = lcm(2*3, 5*2, &primes_under(12)).unwrap();
    assert_eq!(got, 2*3*5);
}