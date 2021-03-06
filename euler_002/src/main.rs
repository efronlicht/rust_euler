/*    Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...


By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.*/

fn main() {
    let fib = Fib::new(4_000_000);
    let evens = fib.filter(|x| x %2 == 0);
    let sum: u32 = evens.sum();
    println!("{}",sum)
}

struct Fib {
    prev: u32,
    current: u32,
    max: u32,
}

impl Fib {
    fn new(max: u32) -> Self {Fib{prev: 0, current: 1, max}}
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current;
        self.current = self.prev + self.current;
        self.prev = temp;
        if temp < self.max {
            Some(temp)
        } else {
            None 
        }
    }
}

