use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let part1_output = part1(input);
    let part2_output = part2(input);
    println!("Part one answer is: {}", part1_output);
    println!("Part two answer is: {}", part2_output);
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut parts = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                check_adjacent(grid.clone(), r, c, &mut visited);
            }
        }
    }

    // Traverse number to the right
    for (r, mut c) in visited {
        let mut num = String::new();
        while c < grid[r].len() && grid[r][c].is_ascii_digit() {
            num.push(grid[r][c]);
            c += 1;
        }
        parts.push(num.parse::<u32>().unwrap());
    }

    parts.iter().sum()
}

fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut gears = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char == '*' {
                // For this part we only want to keep track of the numbers adjacent to a *
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                check_adjacent(grid.clone(), r, c, &mut visited);

                if visited.len() == 2 {
                    let mut g = Vec::new();
                    //Traverse number to the right
                    for (r, mut c) in visited.clone() {
                        let mut num = String::new();
                        while c < grid[r].len() && grid[r][c].is_ascii_digit() {
                            num.push(grid[r][c]);
                            c += 1;
                        }

                        g.push(num.parse::<u32>().unwrap());
                    }
                    gears.push(g[0] * g[1]);
                }
            }
        }
    }

    gears.iter().sum()
}

fn check_adjacent(grid: Vec<Vec<char>>, r: usize, c: usize, visited: &mut HashSet<(usize, usize)>) {
    let rows = grid.len();
    let columns = grid[0].len();
    let neighbors: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1),
    ];

    for (dx, dy) in neighbors {
        let mut c = (c as i32 + dx) as usize;
        let r = (r as i32 + dy) as usize;

        if c < columns && r < rows {
            if grid[r][c].is_ascii_digit() {
                // Find first digit in number
                while c > 0 && grid[r][c - 1].is_ascii_digit() {
                    c -= 1;
                }

                visited.insert((r, c));
            }
        }
    }
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

    #[test]
    fn part2_test() {
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
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
