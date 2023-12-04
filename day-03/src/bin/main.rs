fn main() {
    let input = include_str!("./testinput1.txt");
    let part1_output = part1(input);
    // let part2_output = part2(input);
    println!("Part one answer is: {}", part1_output);
    // println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> u32 {
    let grid: Vec<&str> = input.lines().collect();
    for (r, row) in input.lines().enumerate() {
        for (c, char) in row.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                println!("{},{}: {}", r, c, char);
            }
        }
    }
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
