fn main() {
    fizz_buzz(100)
}

fn fizz_buzz(max: usize) {
    for n in 1..max+1 {
        match (n % 3, n % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", n)
        }
    }
}