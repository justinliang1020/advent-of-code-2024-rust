// template code for each dayXX/mod.rs file
//

use std::{collections::HashMap, task::Wake};

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<&str>) {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    let split: Vec<&str> = input.split("\n\n").collect();
    let (s1, s2) = (split[0], split[1]);

    for s in s1.split_whitespace() {
        let split: Vec<&str> = s.split("|").collect();
        let (n1, n2) = (
            split[0].parse::<u32>().unwrap(),
            split[1].parse::<u32>().unwrap(),
        );
        map.entry(n1).and_modify(|e| e.push(n2)).or_insert(vec![n2]);
    }
    let lines: Vec<&str> = s2.split_whitespace().collect();

    (map, lines)
}

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let (map, lines) = parse_input(input);

    println!("{:?} {:?}", map, lines);
    let mut res = 0;

    for line in lines {
        let l: Vec<&str> = line.split(",").collect();

        let mut correct = true;
        for i in 0..l.len() {
            if !correct {
                break;
            }
            for j in i + 1..l.len() {
                if let Some(v) = map.get(&l[i].parse::<u32>().unwrap()) {
                    if !v.contains(&l[j].parse::<u32>().unwrap()) {
                        correct = false;
                        break;
                    }
                }
            }
        }

        if correct {
            println!("{:?}", l);
            res += l[l.len() / 2].parse::<u32>().unwrap();
        }
    }

    res
    // for each line, iterate through each number
    // for each number, iterate through the rest of the list and check if that pair is in the hashmap
    // if any fail, break out of iteration and don't include that as success
    // if no failures, add the middle element of the array to res
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

    // #[test]
    // fn real_part1() {
    //     println!("Part 1 Output: {}", part1(REAL_INPUT))
    // }
    //
    // #[test]
    // fn real_part2() {
    //     println!("Part 2 Output: {}", part2(REAL_INPUT))
    // }
}
