fn main() {
    let input = include_str!("./example.txt");
    let part1_output = part1(input);
    // let part2_output = part2(input);
    println!("Part one answer is: {}", part1_output);
    // println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> u32 {
    let v: Vec<i32> = input
        .lines()
        .into_iter()
        .map(|val| val.parse::<i32>().unwrap())
        .collect();
    dbg!(v);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(input);
        assert_eq!(result, 114);
    }
}
