struct NrValidator {
	min: u32,
	max: u32,
}
impl NrValidator {
	pub fn nr_validator(s: &String, min: u32, max: u32) -> bool {
		let n = s.parse::<u32>().unwrap();
		n >= min && n <= max
	}
}
impl day4::validator::Validator for NrValidator {
	fn validate(&self, s: &String) -> bool {
		NrValidator::nr_validator(s, self.min, self.max)
	}
}
