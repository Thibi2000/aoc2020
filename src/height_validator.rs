// [[file:~/prog/advent-of-code/2020/README.org::*Heights][Heights:1]]
use crate::nr_validator;
use regex::Regex;

pub fn validate(s: &String, boundaries_cm: (u32, u32), boundaries_in: (u32, u32)) -> bool {
	let last_two: String = s.chars().rev().take(2).collect();
	let re = Regex::new("([0-9]+)[ic][nm]").unwrap();
	let caps = re.captures(s);
	if caps.is_none() {
		return false;
	}
	let digit = String::from(caps.unwrap().get(1).unwrap().as_str());
	if last_two.eq(&String::from("ni")) {
		nr_validator::range_validator(
			&digit,
			boundaries_in.0,
			boundaries_in.1,
		)
	} else {
		nr_validator::range_validator(
			&digit,
			boundaries_cm.0,
			boundaries_cm.1,
		)
	}
}
// Heights:1 ends here
