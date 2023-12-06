fn main() {
    let input = include_str!("./input.txt");
    let part1_output = part1(input);
    // let part2_output = part2(input);
    println!("Part one answer is: {}", part1_output);
    // println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, "todo!()");
    }
}
