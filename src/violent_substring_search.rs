pub fn search(pat: &str, txt: &str) -> i32 {
    'txt_loop: for i in 0..txt.len() {
        // TODO: break if not enough length for pat
        for (j, p) in pat.char_indices() {
            if txt.chars().nth(i + j).unwrap() == p {
                continue;
            } else {
                continue 'txt_loop;
            }
        }
        return i as i32;
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let txt = "ABACADABRAC";
        let pat = "ABRA";
        assert_eq!(search(pat, txt), 6);
    }
}
