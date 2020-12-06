use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod color_validator;
mod eye_color_validator;
mod height_validator;
mod id_validator;
mod nr_validator;

fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

type Solution = fn(Vec<String>) -> (i64, i64);
fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Geef de dag!");
		return;
	}
	let day = args[1].parse::<usize>().unwrap();
	if day < 1 {
		println!("Grapjas");
	}
	let p: String = format!("input/{}", day);
	let path = Path::new(&p);
	let mut solutions: Vec<Solution> = Vec::new();
	solutions.push(day1::solve);
solutions.push(day2::solve);
solutions.push(day3::solve);
solutions.push(day4::solve);
solutions.push(day5::solve);
solutions.push(day6::solve);

	let (a1, a2) = solutions[day - 1](
		read_lines(path)
			.expect(
				"geen lijnen",
			)
			.collect::<Result<Vec<String>, _>>()
			.unwrap(),
	);
	println!("Eerste: {}\nTweede: {}", a1, a2);
}
