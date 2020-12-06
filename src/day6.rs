// [[file:~/prog/advent-of-code/2020/README.org::*Day 6][Day 6:1]]
use std::collections::HashMap;
pub fn solve(lines: Vec<String>) -> (i64, i64) {
	let mut questions_per_group: HashMap<char, i64> = HashMap::new();
	let mut group_len: usize = 0;
	let mut a1: i64 = 0;
	let mut a2: i64 = 0;
	lines.iter().for_each(|line| {
		if line.eq(&String::from("")) {
			a1 += questions_per_group.keys().len() as i64;
			questions_per_group.keys().for_each(|c| {
				if questions_per_group[c] == group_len as i64 {
					a2 += 1;
				}
			});
			questions_per_group = HashMap::new();
			group_len = 0;
		} else {
			line.chars().for_each(|c| {
				// beetje stom, maar kort geschreven :)
				*questions_per_group.entry(c).or_insert(0) += 1;
			});
			group_len += 1;
		}
	});
	a1 += questions_per_group.keys().len() as i64;
	questions_per_group.keys().for_each(|c| {
		if questions_per_group[c] == group_len as i64 {
			a2 += 1;
		}
	});
	(a1, a2)
}
// Day 6:1 ends here
