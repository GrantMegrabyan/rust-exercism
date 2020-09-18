pub fn raindrops(num: u32) -> String {
    let mut result = String::from("");

    if num % 3 == 0 {
        result += "Pling";
    }
    if num % 5 == 0 {
        result += "Plang";
    }
    if num % 7 == 0 {
        result += "Plong";
    }

    if result == "" {
        return num.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pling() {
        assert_eq!("Pling", raindrops(3));
        assert_eq!("Pling", raindrops(93));
        assert_eq!("Pling", raindrops(33));
    }

    #[test]
    fn plang() {
        assert_eq!("Plang", raindrops(5));
        assert_eq!("Plang", raindrops(155));
        assert_eq!("Plang", raindrops(25));
    }

    #[test]
    fn plong() {
        assert_eq!("Plong", raindrops(7));
        assert_eq!("Plong", raindrops(14));
        assert_eq!("Plong", raindrops(77));
    }

    #[test]
    fn pling_plang() {
        assert_eq!("PlingPlang", raindrops(45));
    }

    #[test]
    fn plang_plong() {
        assert_eq!("PlangPlong", raindrops(35));
    }

    #[test]
    fn pling_plang_plong() {
        assert_eq!("PlingPlangPlong", raindrops(105));
    }

    #[test]
    fn nothing() {
        assert_eq!("44", raindrops(44));
        assert_eq!("13", raindrops(13));
        assert_eq!("101", raindrops(101));
    }
}
