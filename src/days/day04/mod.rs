#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word = String::from("XMAS");

    fn inside(x: i32, y: i32, grid: &[Vec<char>]) -> bool {
        0 <= x && x < grid.len() as i32 && 0 <= y && y < grid[0].len() as i32
    }

    let mut res = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != word.chars().next().unwrap() {
                continue;
            }

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let mut good = true;
                    for i in 0..word.len() as i32 {
                        let r2 = r as i32 + dr * i;
                        let c2 = c as i32 + dc * i;
                        // println!("{} {} {} {}", i, r2, c2, inside(r2, c2, &grid));

                        if inside(r2, c2, &grid)
                            && grid[r2 as usize][c2 as usize]
                                == word.chars().nth(i as usize).unwrap()
                        {
                        } else {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0)
    }

    #[test]
    fn real_part1() {
        println!("Part 1 Output: {}", part1(REAL_INPUT))
    }

    // #[test]
    // fn real_part2() {
    //     println!("Part 2 Output: {}", part2(REAL_INPUT))
    // }
}
