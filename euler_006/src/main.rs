/*The sum of the squares of the first ten natural numbers is,

1^2 + 2^2 + ... + 10^2 = 385
The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)^2 = 552 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/
fn main() {
    let sum_of_squares: i64 = (0..101).map(|x| x*x).sum();
    let sum: i64= (0..101).sum();
    let square_of_sum = sum*sum;
    let difference: i64 = (sum_of_squares-square_of_sum).abs();
    println!("{}", difference)
}
