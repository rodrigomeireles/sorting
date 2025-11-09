use clap::{Parser, Subcommand, ValueEnum};
use rand::distr::Uniform;
use rand::prelude::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Clone, ValueEnum)]
enum Algorithms {
    Bubble,
    Merge,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(version, about, long_about = None)]
enum Commands {
    Sort(SortArgs),
    Generate,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct SortArgs {
    /// Name of the sorting algo to run
    #[arg(short, long)]
    algorithm: Algorithms,
}

fn sort(args: SortArgs) {
    let mut l: Vec<usize> = load_numbers_from_file("data.txt");
    match args.algorithm {
        Algorithms::Bubble => bubble_sort(&mut l),
        Algorithms::Merge => merge_sort(&mut l),
    }
    println!("Finished sorting!");
}

/// Function to generate a list of random integers
/// of given size and save them to a file.
fn generate(size: usize) {
    let rng = rand::rng();
    let uniform: Uniform<usize> = Uniform::try_from(0..1000).unwrap();
    let data_iter: std::iter::Take<rand::distr::Iter<Uniform<usize>, ThreadRng, usize>> =
        uniform.sample_iter(rng).take(size);
    // Creates a file and iteratively writes to a buffered writer
    let file = std::fs::File::create("data.txt").expect("Unable to create file");
    let mut writer: BufWriter<File> = std::io::BufWriter::new(file);
    for number in data_iter {
        writer
            .write_all(format!("{} ", number).as_bytes())
            .expect("Unable to write data");
    }
    println!("Generated {} random integers in file data.txt", size);
}

fn load_numbers_from_file(filename: &str) -> Vec<usize> {
    let contents = std::fs::read_to_string(filename).expect("Unable to read file");
    let numbers: Vec<usize> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("Unable to parse number"))
        .collect();
    numbers
}

fn main() {
    let command = Cli::parse().command;
    match command {
        Commands::Sort(args) => sort(args),
        Commands::Generate => generate(100000),
    }
}

fn bubble_sort<T: Ord>(l: &mut [T]) {
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

/// Function receives index of two increasing subarrays
/// and merge them in order
fn merge<T: Ord + Copy + Debug>(l: &mut [T], mid: usize) {
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

fn merge_sort<T: Ord + Copy + Debug>(l: &mut [T]) {
    if l.len() <= 1 {
        return;
    }
    let m = l.len() / 2;
    merge_sort(&mut l[..m]);
    merge_sort(&mut l[m..]);
    merge(l, m);
}

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
        #[cfg(test)]
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut data = vec![1, 3, 2, 4, 8];
        merge(&mut data, 2);
        assert_eq!(data, vec![1, 2, 3, 4, 8]);
    }
}
