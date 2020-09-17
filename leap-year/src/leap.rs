pub fn is_leap_year(year: usize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        assert_eq!(true, is_leap_year(1996));
        assert_eq!(true, is_leap_year(2000));
    }

    #[test]
    fn not_leap_years() {
        assert_eq!(false, is_leap_year(1997));
        assert_eq!(false, is_leap_year(1900));
    }
}
