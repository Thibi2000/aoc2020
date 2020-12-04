// [[file:~/prog/advent-of-code/2020/README.org::*Eye Colors][Eye Colors:1]]
fn eye_color_validator(s: &String) -> bool {
	let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
	colors.contains(&s.as_str())
}
pub fn validate(s: &String) -> bool {
	eye_color_validator(s)
}
// Eye Colors:1 ends here
