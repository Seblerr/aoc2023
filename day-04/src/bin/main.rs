use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let _part1_output = part1(input);
    let part2_output = part2(input);
    // println!("Part one answer is: {}", part1_output);
    println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> u32 {
    let mut ans = 0;
    for line in input.lines() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once('|').unwrap();
        let winners: Vec<u32> = winning_numbers
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();
        let mine: Vec<u32> = my_numbers
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let mut matches: Vec<u32> = Vec::new();
        for w in &winners {
            for m in &mine {
                if w == m {
                    matches.push(*w);
                }
            }
        }

        let num_matches = matches.len();

        ans += if num_matches > 0 {
            2_u32.pow(num_matches as u32 - 1)
        } else {
            0
        };
    }
    ans
}

fn part2(input: &str) -> u32 {
    let mut map: HashMap<usize, u32> = HashMap::new();
    for i in 0..input.lines().count() {
        map.insert(i, 1);
    }
    for (i, line) in input.lines().enumerate() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once('|').unwrap();
        let winners: Vec<u32> = winning_numbers
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();
        let mine: Vec<u32> = my_numbers
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let mut matches: Vec<u32> = Vec::new();
        for w in &winners {
            for m in &mine {
                if w == m {
                    matches.push(*w);
                }
            }
        }

        let num_matches = matches.len();

        for j in i + 1..=i + num_matches {
            if let Some(copies_to_add) = map.get(&i) {
                *map.entry(j).or_insert(1) += *copies_to_add;
            } else {
                println!("You done goofed");
            }
        }
    }
    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result, 30);
    }
}
