fn count_trees(map: &Vec<Vec<char>>, (a, b): (usize, usize)) -> i64 {
	let mut x: usize = a;
	let mut y: usize = b;
	let mut res = 0;
	while y < map.len() {
		if map[y][x] == '#' {
			res += 1;
		}
		x = (x + a) % (map[0].len());
		y += b;
	}
	res
}

pub fn solve(lines: Vec<String>) -> (i64, i64) {
		let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
		let mut res: i64 = 1;
		let vec: Vec<Vec<char>> = lines
			.into_iter()
			.map(|line| line.chars().collect())
			.collect();
		slopes
			.iter()
			.for_each(|slope| res *= count_trees(&vec, *slope));
		(count_trees(&vec, slopes[1]), res)
}
