mod math;

fn main() {
    let factors = [3, 7];
    println!(
        "Sum of multiples {:?} up to {} = {}",
        factors,
        100,
        math::sum_of_multiples(100, &factors)
    );
}
