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

fn iter_merge_sort<T: std::cmp::PartialOrd + Copy>(mut a: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let mid = (lo + hi) / 2;
    iter_merge_sort(&mut a, lo, mid);
    iter_merge_sort(&mut a, mid + 1, hi);
    merge(&mut a, lo, mid, hi);
}

pub fn merge_sort<T: std::cmp::PartialOrd + Copy>(a: &mut Vec<T>) {
    iter_merge_sort(a, 0, a.len() - 1);
}

pub fn merge_bu_sort<T: std::cmp::PartialOrd + Copy>(a: &mut Vec<T>) {
    let mut sz: usize = 1;
    while sz < a.len() {
        let mut lo = 0;
        while lo < a.len() {
            merge(a, lo, lo + sz - 1, (lo + 2 * sz - 1).min(a.len() - 1));
            lo += 2 * sz;
        }
        sz *= 2;
    }
}

pub fn quick_sort<T: std::cmp::PartialOrd + Copy>(a: &mut Vec<T>) {
    quick_part_sort(a, 0, a.len() - 1);
}

fn quick_partition<T: std::cmp::PartialOrd + Copy>(a: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi + 1;
    let v = a[lo];
    loop {
        loop {
            i += 1;
            if a[i] >= v || i == hi {
                break;
            }
        }
        loop {
            j -= 1;
            if v >= a[j] || j == lo {
                break;
            }
        }
        if i >= j {
            break;
        }
        a.swap(i, j);
    }
    a.swap(lo, j);
    return j;
}

fn quick_part_sort<T: std::cmp::PartialOrd + Copy>(a: &mut Vec<T>, lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let j = quick_partition(a, lo, hi);
    quick_part_sort(a, lo, j - 1);
    quick_part_sort(a, j + 1, hi);
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

    #[test]
    fn test_merge_sort() {
        let mut a = create_shuffled_vector(1_000);
        merge_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn test_merge_bu_sort() {
        let mut a = create_shuffled_vector(1_000);
        merge_bu_sort(&mut a);
        println!("{:?}", a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn test_quick_sort() {
        let mut a = create_shuffled_vector(1_000);
        quick_sort(&mut a);
        println!("{:?}", a);
        assert!(is_sorted(&a));
    }
}
