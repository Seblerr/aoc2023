fn main() {
    let input = include_str!("./input.txt");
    let part1_output = part1(input);
    let part2_output = part2(input);
    println!("Part one answer is: {}", part1_output);
    println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut game_possible = true;
        let (game_info, game) = line.split_once(':').unwrap();
        let game_num = game_info.split_once(' ').unwrap().1.parse::<u32>().unwrap();
        let game_parts: Vec<&str> = game.split(';').collect();
        for game in game_parts {
            let cubes: Vec<&str> = game.split(',').collect();
            for c in cubes {
                let (num, color) = c.trim().split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();
                if color == "red" && num > 12
                    || color == "green" && num > 13
                    || color == "blue" && num > 14
                {
                    game_possible = false;
                }
            }
        }
        if game_possible {
            sum += game_num;
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut min_cubes = [0; 3];

        let (_, game) = line.split_once(':').unwrap();
        let game_parts: Vec<&str> = game.split(';').collect();
        for part in game_parts {
            let cubes: Vec<&str> = part.split(',').collect();
            for c in cubes {
                let (num, color) = c.trim().split_once(' ').unwrap();
                if let Ok(number) = num.parse::<u32>() {
                    match color {
                        "red" if number > min_cubes[0] => min_cubes[0] = number,
                        "green" if number > min_cubes[1] => min_cubes[1] = number,
                        "blue" if number > min_cubes[2] => min_cubes[2] = number,
                        _ => {}
                    }
                }
            }
        }
        sum += min_cubes.iter().product::<u32>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("./testinput1.txt");
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("./testinput1.txt");
        let result = part2(input);
        assert_eq!(result, 2286);
    }
}
