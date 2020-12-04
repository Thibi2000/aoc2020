// [[file:~/prog/advent-of-code/2020/README.org::*Hair Colors][Hair Colors:1]]
use regex::Regex;
fn color_validator(s: &String) -> bool {
	let color_regex: Regex = Regex::new("#[0-9a-z]{6}").unwrap();
	color_regex.is_match(s)
}
pub fn validate(s: &String) -> bool {
	color_validator(s)
}
// Hair Colors:1 ends here
