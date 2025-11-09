use std::fmt::Debug;

pub fn bubble_sort<T: Ord>(l: &mut [T]) {
    let n = l.len();
    if n == 0 {
        return;
    }
    let mut swapped: bool = true;
    while swapped {
        swapped = false;
        for i in 0..n - 1 {
            if l[i] > l[i + 1] {
                swapped = true;
                l.swap(i, i + 1);
            }
        }
    }
}

pub fn merge<T: Ord + Copy + Debug>(l: &mut [T], mid: usize) {
    let mut work: Vec<T> = Vec::new();
    let mut left = 0;
    let mut right = mid;
    while work.len() < l.len() {
        if l[left] <= l[right] {
            work.push(l[left]);
            left += 1;
        } else {
            work.push(l[right]);
            right += 1;
        }
        if left >= mid {
            work.extend_from_slice(&l[right..]);
        }
        if right >= l.len() {
            work.extend_from_slice(&l[left..mid]);
        }
    }
    l.copy_from_slice(&work);
}

pub fn merge_sort<T: Ord + Copy + Debug>(l: &mut [T]) {
    if l.len() <= 1 {
        return;
    }
    let m = l.len() / 2;
    merge_sort(&mut l[..m]);
    merge_sort(&mut l[m..]);
    merge(l, m);
}
