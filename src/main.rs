use clap::{Parser, Subcommand, ValueEnum};
use rand::distr::Uniform;
use rand::prelude::*;
use sorting::{bubble_sort, merge_sort};
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
    let contents = std::fs::read_to_string(filename).expect(
        "Unable to read file. You may need to generate data first. Run `sorting generate`!",
    );
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
