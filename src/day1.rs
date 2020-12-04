use std::collections::HashMap;

fn get_numbers(list: &Vec<i64>, number: i64) -> Result<(i64, i64), String> {
	let mut map = HashMap::new();
	for n in list {
		let diff: i64 = number - n;
		if diff > 0 {
			if map.contains_key(&diff) {
				return Ok((*n, diff));
			}
			map.insert(n, diff);
		}
	}
	Err("Geen paar".to_string())
}

fn get_trio(list: &Vec<i64>, number: i64) -> Result<(i64, i64, i64), String> {
	for n in list {
		if let Ok((x, y)) = get_numbers(&list, number - n) {
			return Ok((*n, x, y));
		}
	}
	Err("Geen trio".to_string())
}

pub fn solve(lines: Vec<String>) -> (i64, i64) {
	let numbers: Vec<i64> =
		lines.iter().map(|s| s.parse::<i64>().unwrap()).collect();
	let res1 = get_numbers(&numbers, 2020).unwrap();
	let res2 = get_trio(&numbers, 2020).unwrap();
	(res1.0 * res1.1, res2.0 * res2.1 * res2.2)
}
