struct Empty;

impl Iterator for Empty {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        return None;
    }
}

struct TheAnswer;

impl Iterator for TheAnswer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}

struct Counter {
    limit: u32,
    current: u32,
}

impl Counter {
    pub fn new(limit: u32) -> Counter {
        Counter { limit, current: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.limit {
            None
        } else {
            self.current += 1;
            Some(self.current - 1)
        }
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let old_curr = self.curr;
        let old_next = self.next;

        match old_curr.checked_add(old_next) {
            // Overflow
            None => None,

            Some(new_next) => {
                self.curr = old_next;
                self.next = new_next;
                Some(old_curr)
            }
        }
    }
}

struct Doubler<I> {
    iter: I,
}

impl<I> Iterator for Doubler<I>
where
    I: Iterator,
    I::Item: std::ops::Add<Output = I::Item> + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(x) => Some(x + x),
        }
    }
}

fn main() {
    for i in Empty {
        panic!("This should not happen!");
    }

    for i in TheAnswer.take(10) {
        println!("The answer is {}", i);
    }
    println!("");

    for i in Counter::new(7) {
        println!("Counter: {}", i);
    }

    println!("");
    for i in Fibonacci::new().take(5) {
        print!("{} ", i);
    }
    println!("");

    println!("");
    let doubler = Doubler {
        iter: Counter::new(10),
    };
    for i in doubler {
        println!("Doubled - {}", i);
    }
    println!("");
    
    let sum = (1..10).fold(0, |s, x| s + x);
    println!("Sum = {}", sum);

    println!("All done!");
}
