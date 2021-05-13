pub fn horner(dividend: impl Iterator<Item = u32>, divisor: u32, base: u32) -> u32 {
    let mut remainder = 0;
    for i in dividend {
        remainder = (remainder * base + i) % divisor;
    }
    remainder
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let dividend = "26535".chars().map(|c| c as u32 - '0' as u32);
        assert_eq!(horner(dividend, 997, 10), 613);
    }
}
