use regex::Regex;

#[derive(Default)]
struct Foo {
	min: i8,
	max: i8,
	c: char,
	password: String,
}

fn is_valid_password1(f: &Foo) -> bool {
	let freq = f.password.chars().filter(|c| *c == f.c).count() as i8;
	return f.min <= freq && freq <= f.max;
}

fn is_valid_password2(f: &Foo) -> bool {
	let bool1 = f.password.as_bytes()[(f.min - 1) as usize] as char == f.c;
	let bool2 = f.password.as_bytes()[(f.max - 1) as usize] as char == f.c;
	bool1 ^ bool2
}

// string is van de vorm :
// i8-i8 char: String
fn parse_line(re: &Regex, line: String) -> Option<Foo> {
	let mut res: Foo = Default::default();
	let caps = re.captures(&line)?;
	res.min = caps.get(1)?.as_str().parse::<i8>().unwrap();
	res.max = caps.get(2)?.as_str().parse::<i8>().unwrap();
	res.c = caps.get(3)?.as_str().parse::<char>().unwrap();
	res.password = String::from(caps.get(4)?.as_str());
	Some(res)
}

pub fn solve(lines: Vec<String>) -> (i64, i64) {
	let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

	let mut aantal1 = 0;
	let mut aantal2 = 0;
	lines
		.iter()
		.map(|line| parse_line(&re, line.to_string()))
		.for_each(|foo| {
			if let Some(f) = foo {
				if is_valid_password1(&f) {
					aantal1 += 1;
				}
				if is_valid_password2(&f) {
					aantal2 += 1;
				}
			}
		});
	(aantal1, aantal2)
}
