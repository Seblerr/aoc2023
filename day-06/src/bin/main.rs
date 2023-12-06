use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = solution(input);
    println!("Answer is: {}", output);
}

fn solution(input: &str) -> u32 {
    let times = input.lines().nth(0).unwrap();
    let distances = input.lines().nth(1).unwrap();

    let re = Regex::new(r"\d+").unwrap();

    let times: Vec<_> = re
        .find_iter(times)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u64> = re
        .find_iter(distances)
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    let mut ans: Vec<u32> = vec![0; times.len()];

    for (i, &time) in times.iter().enumerate() {
        let record = distances[i];
        for j in 1..=time {
            let distance = j as u64 * (time as u64 - j as u64);
            if distance > record {
                ans[i] += 1;
            }
        }
    }

    ans.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = solution(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn part2_test() {
        let input = "Time:      71530
Distance:  940200";
        let result = solution(input);
        assert_eq!(result, 71503);
    }
}
