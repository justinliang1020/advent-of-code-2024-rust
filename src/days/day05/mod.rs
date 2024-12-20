// template code for each dayXX/mod.rs file
//

use std::collections::{HashMap, HashSet};

fn parse_input_set(input: &str) -> (HashSet<(&str, &str)>, Vec<&str>) {
    let mut set = HashSet::new();

    let split: Vec<&str> = input.split("\n\n").collect();
    let (s1, s2) = (split[0], split[1]);

    for s in s1.split_whitespace() {
        // format i.e. 12|86
        let split = s.split("|").collect::<Vec<&str>>();
        set.insert((split[0], split[1]));
    }
    let lines: Vec<&str> = s2.split_whitespace().collect();

    (set, lines)
}

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let (set, lines) = parse_input_set(input);
    let mut res = 0;

    for line in lines {
        let line: Vec<&str> = line.split(",").collect();

        let mut correct = true;
        for i in 0..line.len() {
            if !correct {
                break;
            }
            for j in i + 1..line.len() {
                if !set.contains(&(line[i], line[j])) {
                    correct = false;
                    break;
                }
            }
        }

        if correct {
            res += line[line.len() / 2].parse::<u32>().unwrap();
        }
    }

    res
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    // create map of frequency for each correct line
    // then generate order by getting most element wiht highest number of overlapping values in the
    // map
    // whatever
    let (set, lines) = parse_input_set(input);
    let mut res = 0;

    for line in lines {
        let line: Vec<&str> = line.split(",").collect();

        let mut correct = true;
        for i in 0..line.len() {
            if !correct {
                break;
            }
            for j in i + 1..line.len() {
                if !set.contains(&(line[i], line[j])) {
                    correct = false;
                    break;
                }
            }
        }

        if !correct {
            // do stuff
            let mut freqs: Vec<(&str, usize)> = vec![];

            for i in 0..line.len() {
                let mut line_set: HashSet<(&str, &str)> = HashSet::new();
                for j in 0..line.len() {
                    if i == j {
                        continue;
                    }
                    line_set.insert((line[i], line[j]));
                }
                let intersection_count = line_set.intersection(&set).count();
                freqs.push((line[i], intersection_count))
            }

            freqs.sort_by_key(|k| k.1);
            res += freqs[freqs.len() / 2].0.parse::<u32>().unwrap();
        }
    }

    res
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
        assert_eq!(part2(TEST_INPUT), 123)
    }

    #[test]
    fn real_part1() {
        println!("Part 1 Output: {}", part1(REAL_INPUT))
    }

    #[test]
    fn real_part2() {
        println!("Part 2 Output: {}", part2(REAL_INPUT))
    }
}
