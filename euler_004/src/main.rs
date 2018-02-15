/*A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.*/

fn main() {
    println!("{}", largest_three_digit_palindrome())
}

fn largest_three_digit_palindrome() -> u64 {
    for n in (100..1000).rev() {
        for m in (100..n+1).rev() {
            let x = n*m;
            if is_palindrome(x) {
                return x;
            };
        };
    };
    0 // this won't happen
}


fn number_of_digits(x: u64) -> u64 {
    let mut digits: u64 = 0;
    while u64::pow(10, digits as u32) < x {
        digits += 1;
    };
    digits
}

fn nth_digit(x: u64, n: u64 ) -> u64 {
   let digit_floor = u64::pow(10, number_of_digits(n) as u32);
   let r = x % u64::pow(10, digit_floor as u32);
   ((x - r) / digit_floor) as u64
}

fn is_palindrome(x: u64) -> bool {
    let mut i = 0;
    let mut j = number_of_digits(x);
    while i < j {
        if nth_digit(x, i) != nth_digit(x, j) {
            return false
        }
        i = i+1;
        j = j-1;
    }
    true
}