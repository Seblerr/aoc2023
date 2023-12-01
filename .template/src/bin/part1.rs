fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = include_str!("./testinput.txt");
        let result = part1(input);
        assert_eq!(result, 0);
    }
}
