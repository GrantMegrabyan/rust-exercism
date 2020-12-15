pub fn descending_order(x: u64) -> u64 {
    let mut v: Vec<u64> = vec![];
    let mut tmp = x;
    while tmp > 0 {
        let r = tmp % 10;
        tmp = tmp / 10;
        v.push(r);
    }

    v.sort();
    v.iter().rev().fold(0, |s, d| s * 10 + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}