use itertools::Itertools;

fn main() {
    let source_file = std::env::args()
        .nth(1)
        .expect("Please supply a file to read from");

    let number_format = regex::Regex::new(r"^-?\d+\.\d{2,}$").expect("Invalid regex");

    let mut numbers = std::fs::read_to_string(source_file)
        .expect("Could not read file")
        .lines()
		.filter(|line| !line.is_empty())
        .inspect(|line| assert!(number_format.is_match(line), "Invalid number: {}", line))
        .map(|line| line.replace(".", ""))
        .map(|line| {
            line.parse::<i64>()
                .expect(format!("Could not parse number {}", line).as_str())
        })
        .collect_vec();

	numbers.sort();

	numbers
		.iter()
		.powerset()
		.filter(|set| !set.is_empty())
		.filter(|set| set.into_iter().map(|n| *n).sum::<i64>() == 0)
		.map(|set| set.into_iter().map(|num| *num as f64 / 100 as f64).collect_vec())
		.for_each(|element| println!("{:.2?}", element))
}
