// [[file:~/prog/advent-of-code/2020/README.org::*Day 4][Day 4:1]]
use regex::Regex;
use std::collections::HashMap;

use crate::color_validator;
use crate::eye_color_validator;
use crate::height_validator;
use crate::id_validator;
use crate::nr_validator;

type Passport = HashMap<String, String>;
fn has_required_fields(p: &Passport) -> bool {
	let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	// let optional = vec!["cid"];
	for r in required {
		if !p.contains_key(r) {
			return false;
		}
	}
	true
}

fn has_valid_data(p: &Passport) -> bool {
	for (key, value) in p {
		let b: bool = match key.as_str() {
			"byr" => nr_validator::validate(value, 1920, 2002),
			"iyr" => nr_validator::validate(value, 2010, 2020),
			"eyr" => nr_validator::validate(value, 2020, 2030),
			"hgt" => height_validator::validate(value, (150, 193), (56, 79)),
			"hcl" => color_validator::validate(value),
			"ecl" => eye_color_validator::validate(value),
			"pid" => id_validator::validate(value),
			_ => continue
		};
		if !b {
			return b;
		}
	}
	return true;
}

pub fn solve(lines: Vec<String>) -> (i64, i64) {
	let re = Regex::new("([a-z]+):([a-z#0-9]+)").unwrap();
	let mut passports: Vec<Passport> = Vec::new();
	let mut current: Passport = Passport::new();
	lines.iter().for_each(|line| {
		if line.eq(&String::from("")) {
			passports.push(current.clone());
			current = Passport::new();
		} else {
			line.split(" ")
				.map(|field| re.captures(&field))
				.collect::<Vec<Option<regex::Captures>>>()
				.iter()
				.for_each(|capture| {
					current.insert(
						capture
							.as_ref()
							.expect("no captures")
							.get(1)
							.unwrap()
							.as_str()
							.to_string(),
						capture
							.as_ref()
							.expect("no captures")
							.get(2)
							.unwrap()
							.as_str()
							.to_string(),
					);
				});
		}
	});
	passports.push(current.clone());
	let all_fields = passports
		.iter()
		.filter(|p| has_required_fields(p))
		.collect::<Vec<&Passport>>();
	let valid_data = passports
		.iter()
		.filter(|p| has_required_fields(p) && has_valid_data(p))
		.collect::<Vec<&Passport>>();
	(all_fields.len() as i64, valid_data.len() as i64)
}
// Day 4:1 ends here
