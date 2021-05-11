pub struct BoyerMoore<'a> {
    pattern: &'a str,
    right: Vec<i32>,
}

impl<'a> BoyerMoore<'a> {
    pub fn new(pattern: &'a str) -> Self {
        let mut right = vec![-1; 256];
        for (i, c) in pattern.char_indices() {
            right[c as usize] = i as i32;
        }
        Self { pattern, right }
    }

    pub fn search(self, txt: &str) -> i32 {
        let mut i = 0;
        while i <= txt.len() - self.pattern.len() {
            let mut skip = 0;
            for (j, c) in self.pattern.char_indices().rev() {
                if c == txt.chars().nth(i + j).unwrap() {
                    continue;
                }
                skip = 1.max(j as i32 - self.right[txt.chars().nth(i + j).unwrap() as usize]);
                break;
            }
            if skip == 0 {
                return i as i32;
            }
            i += skip as usize;
        }
        self.pattern.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_works() {
        assert_eq!(
            BoyerMoore::new("NEEDLE").search("FINDINAHAYSTACKNEEDLE"),
            15
        );
    }
}
