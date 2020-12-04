pub fn range_validator(s: &String, min: u32, max: u32) -> bool {
	let n = s.parse::<u32>().unwrap();
	n >= min && n <= max
}

pub fn validate(s: &String, min: u32, max: u32) -> bool {
	return range_validator(s, min, max);
}
