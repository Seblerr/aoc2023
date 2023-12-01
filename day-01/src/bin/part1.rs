fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("Part one answer is: {}", output);
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

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        // let result = part1(include_str!("./testinput.txt"));
        // assert_eq!(result, "142");
    }
}
