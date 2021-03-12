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

pub fn shell_sort<T: std::cmp::PartialOrd>(a: &mut Vec<T>) {
    let length = a.len();
    let mut h: usize = 1;
    while h < length / 3 {
        h = 3 * h + 1;
    }
    while h >= 1 {
        for i in h..length {
            let mut j = i;
            while j >= h && a[j] < a[j - h] {
                a.swap(j, j - h);
                j -= h;
            }
        }
        h /= 3;
    }
}

/// assume that both left and right parts are sorted
fn merge<T: std::cmp::PartialOrd + Copy>(a: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    let mut i = lo;
    let mut j = mid + 1;
    let aux = a.clone();
    let mut k = lo;
    while k <= hi {
        if i > mid {
            a[k] = aux[j];
            j += 1;
        } else if j > hi {
            a[k] = aux[i];
            i += 1;
        } else if aux[i] < aux[j] {
            a[k] = aux[i];
            i += 1;
        } else {
            a[k] = aux[j];
            j += 1;
        }
        k += 1;
    }
}

#[cfg(test)]
mod test {
    use rand::prelude::SliceRandom;

    use super::*;

    fn is_sorted<T: std::cmp::PartialOrd>(a: &Vec<T>) -> bool {
        for i in 1..a.len() {
            if a[i - 1] > a[i] {
                return false;
            }
        }
        true
    }

    fn create_shuffled_vector(length: i32) -> Vec<i32> {
        let mut a: Vec<i32> = (0..length).collect();
        a.shuffle(&mut rand::thread_rng());
        a
    }

    #[test]
    fn test_selection_sort() {
        let mut a = create_shuffled_vector(1_000);
        selection_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn test_insertion_sort() {
        let mut a = create_shuffled_vector(1_000);
        insertion_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn test_shell_sort() {
        let mut a = create_shuffled_vector(1_000);
        shell_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn test_merge() {
        let mut a = "EEGMRACERT".chars().collect(); // cSpell: disable-line
        merge(&mut a, 0, 4, 9);
        assert!(is_sorted(&a));
    }
}
