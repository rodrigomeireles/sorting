use sorting::{bubble_sort, merge, merge_sort};

type SortFn<T> = fn(&mut [T]);

fn test_sort_basic(sort_fn: SortFn<i32>) {
    let mut data = vec![5, 3, 8, 4, 2];
    sort_fn(&mut data);
    assert_eq!(data, vec![2, 3, 4, 5, 8]);
}

fn test_sort_already_sorted(sort_fn: SortFn<i32>) {
    let mut data = vec![1, 2, 3, 4, 5];
    sort_fn(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

fn test_sort_reverse_sorted(sort_fn: SortFn<i32>) {
    let mut data = vec![9, 7, 5, 3, 1];
    sort_fn(&mut data);
    assert_eq!(data, vec![1, 3, 5, 7, 9]);
}

fn test_sort_with_duplicates(sort_fn: SortFn<i32>) {
    let mut data = vec![4, 2, 4, 1, 3, 2];
    sort_fn(&mut data);
    assert_eq!(data, vec![1, 2, 2, 3, 4, 4]);
}

fn test_sort_empty(sort_fn: SortFn<i32>) {
    let mut data: Vec<i32> = vec![];
    sort_fn(&mut data);
    assert_eq!(data, vec![]);
}

fn test_sort_single_element(sort_fn: SortFn<i32>) {
    let mut data = vec![42];
    sort_fn(&mut data);
    assert_eq!(data, vec![42]);
}

macro_rules! generate_sort_tests {
    ($modname:ident, $sortfn:ident) => {
        mod $modname {
            use super::*;
            #[test]
            fn basic() {
                super::test_sort_basic($sortfn);
            }
            #[test]
            fn already_sorted() {
                super::test_sort_already_sorted($sortfn);
            }
            #[test]
            fn reverse_sorted() {
                super::test_sort_reverse_sorted($sortfn);
            }
            #[test]
            fn with_duplicates() {
                super::test_sort_with_duplicates($sortfn);
            }
            #[test]
            fn empty() {
                super::test_sort_empty($sortfn);
            }
            #[test]
            fn single_element() {
                super::test_sort_single_element($sortfn);
            }
        }
    };
}

generate_sort_tests!(bubble_sort_tests, bubble_sort);
generate_sort_tests!(merge_sort_tests, merge_sort);

#[test]
fn test_merge() {
    let mut data = vec![1, 3, 2, 4, 8];
    merge(&mut data, 2);
    assert_eq!(data, vec![1, 2, 3, 4, 8]);
}
