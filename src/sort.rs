pub fn selection_sort<T: std::cmp::PartialOrd>(a: &mut Vec<T>) {
    for i in 0..a.len() {
        let mut min = &a[i];
        let mut min_index = i;
        for j in i + 1..a.len() {
            if a[j] < *min {
                min_index = j;
                min = &a[j];
            }
        }
        if i != min_index {
            a.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut a = vec![3, 1, 2, 5, 4];
        selection_sort(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }
}
