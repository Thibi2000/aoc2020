// [[file:~/prog/advent-of-code/2020/README.org::*Day 7][Day 7:1]]
use regex::Regex;
use std::collections::HashMap;

pub fn solve(lines: Vec<String>) -> (i64, i64) {
	let re = Regex::new("([0-9]*) ?([a-z]+ [a-z]+) [a-z]+").unwrap();
	let mut to_parents: HashMap<String, Vec<String>> = HashMap::new();
	let mut to_children: HashMap<String, Vec<(String, u64)>> = HashMap::new();
	let my_bag: String = "shiny gold".to_string();
	lines.iter().for_each(|line| {
		let bags: Vec<&str> = line.split("contain").collect();
		assert_eq!(bags.len(), 2);
		let parent: String = String::from(
			re.captures(bags[0])
				.expect("No captures for parent")
				.get(2)
				.expect("there's no second index for parent capture")
				.as_str(),
		);
		to_parents.entry(parent.clone()).or_insert(Vec::new());
		re.captures_iter(bags[1]).for_each(|child| {
			if let Ok(amount) = child[1].parse::<u64>() {
				to_children
					.entry(parent.clone())
					.and_modify(|v| v.push((child[2].to_string(), amount)))
					.or_insert(vec![(child[2].to_string(), amount)]);
				to_parents
					.entry(child[2].to_string())
					.and_modify(|v| v.push(parent.clone()))
					.or_insert(vec![parent.clone()]);
			}
		});
	});

	let mut valid_bags: Vec<String> = get_bags(&to_parents, &my_bag);
	valid_bags.sort();
	valid_bags.dedup();
	return (
		valid_bags.len() as i64,
		get_amount_bags(&to_children, &my_bag) as i64 - 1,
	);
}

fn get_bags(g: &HashMap<String, Vec<String>>, s: &String) -> Vec<String> {
	let mut res: Vec<String> = Vec::new();
	for p in &g[s] {
		if !res.contains(&p) {
			res.push(p.to_string());
		}
	}
	let v: Vec<String> = res.clone();
	for b in &v {
		res = [res, get_bags(&g, &b)].concat();
	}
	res
}

fn get_amount_bags(g: &HashMap<String, Vec<(String, u64)>>, s: &String) -> u64 {
	let mut res: u64 = 1;
	if g.contains_key(s) {
		for p in &g[s] {
			res += p.1 * get_amount_bags(g, &p.0);
		}
	}
	res
}
// Day 7:1 ends here
