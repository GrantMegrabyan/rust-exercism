mod clock;

fn main() {
    let clock = clock::Clock::new(1000, 200);
    println!("{}", clock);
    
    let clock2 = clock.add_minutes(2000);
    println!("{}", clock2);
}
