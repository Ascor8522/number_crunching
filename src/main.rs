use float_cmp::approx_eq;
use itertools::Itertools;
use regex::Regex;
use std::env::args;
use std::fs::read_to_string;
use std::num::ParseFloatError;
use std::panic;
use std::vec::Vec;

fn main() -> () {
    setup_panic();

    let source_file = args().nth(1).expect("Please supply a file to read from");
    let sum_target = args()
        .nth(2)
        .unwrap_or(String::from("0"))
        .parse::<f64>()
        .expect("Please supply a valid sum target");

    let _decimal_places = 2;
    let number_format = Regex::new(r"^-?\d+\.\d{2,}$").expect("Invalid regex");
    let assertion = |str: &str| number_format.is_match(str).then_some(()).ok_or(());
    let parse = |str: &str| str.parse::<f64>();

    let mut numbers = read_numbers_from_file(&source_file, &assertion, &parse);
    numbers.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());

    let subsets = find_subsets(&numbers, sum_target);

    let uniques = remove_duplicates(&subsets);

    display_results(&uniques);
}

/// Sets up the panic hook to print a nice panic message.
fn setup_panic() -> () {
    panic::set_hook(Box::new(move |info| println!("Error: {}", info.payload().downcast_ref::<String>().unwrap())))
}

/// Reads numbers from a file
/// checks they match a specific format
/// parses them
/// and returns them in a vector.
fn read_numbers_from_file(
    file: &String,
    assertion: &impl Fn(&str) -> Result<(), ()>,
    parse: &impl Fn(&str) -> Result<f64, ParseFloatError>,
) -> Vec<f64> {
    read_to_string(file)
        .expect("Could not read file")
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .inspect(|line| assertion(*line).expect(format!("Invalid number: {}", line).as_str()))
        .map(|line| parse(line).expect(format!("Could not parse number {}", line).as_str()))
        .collect_vec()
}

/// Finds all subsets of a vector of numbers that sum to a target sum.
fn find_subsets(numbers: &Vec<f64>, sum: f64) -> Vec<Vec<f64>> {
    numbers
        .iter()
        .powerset()
        .skip(1) // First set is always empty
        .map(|set| set.into_iter().map(|num| *num).collect_vec())
        .filter(|set| {
            approx_eq!(
                f64,
                set.iter().sum::<f64>(),
                sum,
                epsilon = set.len() as f64 * f64::EPSILON
            )
        })
        .collect_vec()
}

/// Removes the duplicate subsets from a vector of subsets.
fn remove_duplicates<'a>(numbers: &Vec<Vec<f64>>) -> Vec<&Vec<f64>> {
    numbers
        .into_iter()
        .fold(Vec::new(), |mut smaller_vecs, bigger_vec| {
            if !smaller_vecs.iter().any(|smaller_vec| {
                smaller_vec
                    .iter()
                    .all(|smaller_vec_element| bigger_vec.contains(smaller_vec_element))
            }) {
                smaller_vecs.push(bigger_vec);
            }
            smaller_vecs
        })
}

/// Displays the results of the program.
fn display_results(results: &Vec<&Vec<f64>>) -> () {
    results
        .iter()
        .for_each(|element| println!("{:.2?}", element));
}
