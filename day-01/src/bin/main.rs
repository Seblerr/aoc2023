fn main() {
    let input = include_str!("./input.txt");
    let part1_output = part1(input);
    let part2_output = part2(input);
    println!("Part one answer is: {}", part1_output);
    println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let digits: String = line.chars().filter(|p| p.is_ascii_digit()).collect();
        let line_sum = format!(
            "{}{}",
            digits.chars().next().unwrap(),
            digits.chars().last().unwrap()
        )
        .parse::<u32>()
        .unwrap();
        sum += line_sum;
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = String::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.push(c)
            }
            let numbers_as_strings = vec![
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            for (j, num) in numbers_as_strings.iter().enumerate() {
                if line.chars().skip(i).collect::<String>().starts_with(num) {
                    digits.push_str(&(j + 1).to_string())
                }
            }
        }
        let line_sum = format!(
            "{}{}",
            digits.chars().next().unwrap(),
            digits.chars().last().unwrap()
        )
        .parse::<u32>()
        .unwrap();
        sum += line_sum;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(include_str!("./testinput.txt"));
        assert_eq!(result, "281");
    }
}
