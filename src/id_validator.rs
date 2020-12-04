// [[file:~/prog/advent-of-code/2020/README.org::*Pid][Pid:1]]
use regex::Regex;
fn id_validator(s: &String) -> bool {
	let id_regex = Regex::new("^[0-9]{9}$").unwrap();
	id_regex.is_match(s)
}

pub fn validate(s: &String) -> bool {
	id_validator(s)
}
// Pid:1 ends here
