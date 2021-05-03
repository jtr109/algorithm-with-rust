pub fn sort(mut a: Vec<String>, w: usize) -> Vec<String> {
    let range: usize = 256;
    for d in (0..w).rev() {
        let mut aux = vec![String::new(); a.len()];
        // 从低位开始进行键索引计数法
        let mut count = vec![0usize; range + 1];
        for i in 0..a.len() {
            // 计算出现频率
            count[a[i].chars().nth(d).unwrap() as usize + 1] += 1;
        }
        for r in 0..range {
            // 将频率转换为索引
            count[r + 1] += count[r];
        }
        for i in 0..a.len() {
            // 将元素分类
            aux[count[a[i].chars().nth(d).unwrap() as usize]] = a[i].clone();
            count[a[i].chars().nth(d).unwrap() as usize] += 1;
        }
        for i in 0..a.len() {
            a[i] = aux[i].clone();
        }
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_as_expected() {
        let input = vec![
            "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
            "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let expected: Vec<String> = vec![
            "1ICK750", "1ICK750", "1OHV845", "1OHV845", "1OHV845", "2IYE230", "2RLA629", "2RLA629",
            "3ATW723", "3CIO720", "3CIO720", "4JZY524", "4PGC938",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let a = sort(input, 7);
        assert_eq!(a, expected);
    }
}
