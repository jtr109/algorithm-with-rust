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

pub fn insertion_sort<T: std::cmp::PartialOrd>(a: &mut Vec<T>) {
    for i in 0..a.len() {
        for j in (0..i).rev() {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
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

    #[test]
    fn test_insertion_sort() {
        let mut a = vec![3, 1, 2, 5, 4];
        insertion_sort(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }
}
