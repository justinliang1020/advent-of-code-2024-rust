use regex::Regex;

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    let mut sum: u32 = 0;
    let caps = re.captures_iter(input);
    for c in caps {
        sum += c["x"].parse::<u32>().unwrap() * c["y"].parse::<u32>().unwrap();
    }
    sum
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))")
        .unwrap();
    let mut sum: u32 = 0;
    let caps = re.captures_iter(input);
    let mut enabled = true;
    for c in caps {
        // c.get(0) returns the entire match
        match c.get(0).unwrap().as_str() {
            r"do()" => enabled = true,
            r"don't()" => enabled = false,
            _ => {
                if enabled {
                    // kinda unsafe?
                    sum += c["x"].parse::<u32>().unwrap() * c["y"].parse::<u32>().unwrap();
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = include_str!("test.txt");
    const TEST2_INPUT: &str = include_str!("test2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST2_INPUT), 48)
    }

    #[test]
    fn real_part1() {
        println!("Part 1 Output: {}", part1(REAL_INPUT))
    }

    #[test]
    fn real_part2() {
        println!("Part 2 Output: {}", part2(REAL_INPUT))
    }

    #[test]
    fn real_test_part1() {
        assert_eq!(part1(REAL_INPUT), 166905464)
    }
}
