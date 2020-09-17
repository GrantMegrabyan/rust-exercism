mod leap;

fn main() {
    for year in 1900..2000 {
        if leap::is_leap_year(year) {
            println!("{} was a leap year", year);
        }
    }
}
