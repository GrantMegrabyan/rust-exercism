mod raindrops;

fn main() {
    for num in 1..110 {
        let sound = raindrops::raindrops(num);
        if sound != format!("{}", num) {
            println!("{}: {}", num, sound);
        }
    }
}
