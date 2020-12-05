// [[file:~/prog/advent-of-code/2020/README.org::*Day 5][Day 5:1]]
pub fn solve(lines: Vec<String>) -> (i64, i64) {
	let mut ids: Vec<i64> = lines.iter().map(|line| {
		let mut row: (u8, u8) = (0, 127);
		let mut row_m = row.1 / 2;
		let mut col: (u8, u8) = (0, 7);
		let mut col_m = col.1 / 2;
		line.chars().for_each(|c| {
			match c {
				'F' => {
					row = (row.0, row_m);
				}
				'B' => {
					row = (row_m, row.1);
				}
				'L' => col = (col.0, col_m),
				'R' => col = (col_m, col.1),
				_ => println!("Wrong character in input"),
			}
			row_m = (row.0 + row.1) / 2;
			col_m = (col.0 + col.1) / 2;
		});
		row.1 as i64 * 8 + col.1 as i64
	}).collect();
	ids.sort();
	let mut prev = ids[0];
	let mut your_id: i64 = 0;
	ids.iter().for_each(|n| {
		if n - prev == 2 {
			your_id = prev + 1;
		}
		prev = *n;
	});
	(ids[ids.len() - 1], your_id)
}
// Day 5:1 ends here
