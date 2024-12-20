// template code for each dayXX/mod.rs file
//

use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<String>, Vec<&str>) {
    let mut set = HashSet::new();

    let split: Vec<&str> = input.split("\n\n").collect();
    let (s1, s2) = (split[0], split[1]);

    for s in s1.split_whitespace() {
        // format i.e. 12|86
        set.insert(s.to_string());
    }
    let lines: Vec<&str> = s2.split_whitespace().collect();

    (set, lines)
}

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let (set, lines) = parse_input(input);
    let mut res = 0;

    for line in lines {
        let l: Vec<&str> = line.split(",").collect();

        let mut correct = true;
        for i in 0..l.len() {
            if !correct {
                break;
            }
            for j in i + 1..l.len() {
                let entry = format!("{}|{}", l[i], l[j]);
                if !set.contains(&entry) {
                    correct = false;
                    break;
                }
            }
        }

        if correct {
            res += l[l.len() / 2].parse::<u32>().unwrap();
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
        assert_eq!(part1(TEST_INPUT), 143)
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
