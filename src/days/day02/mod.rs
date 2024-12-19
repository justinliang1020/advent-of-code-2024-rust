#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let mut is_line_increasing: Option<bool> = None;
        let mut failed = false;
        for (prev, current) in numbers.iter().zip(numbers.iter().skip(1)) {
            if prev == current {
                failed = true;
                break;
            }

            let prev_current_diff = prev.abs_diff(*current);
            if !(1..=3).contains(&prev_current_diff) {
                failed = true;
                break;
            }

            match is_line_increasing {
                Some(true) => {
                    if prev > current {
                        failed = true;
                        break;
                    }
                }
                Some(false) => {
                    if prev < current {
                        failed = true;
                        break;
                    }
                }
                None => {
                    is_line_increasing = Some(*prev < *current);
                }
            }
        }
        if !failed {
            res += 1;
            // println!("{:?}", original)
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
        assert_eq!(part1(TEST_INPUT), 2)
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
