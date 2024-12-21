use std::collections::VecDeque;

// template code for each dayXX/mod.rs file
enum Operator {
    Add,
    Multiply,
}
#[allow(dead_code)]
fn part1(input: &str) -> u64 {
    // parse the input, iterate on each line
    // iterate through each of the operands, do a brute force branching pattern testing each
    // operand
    fn is_valid(
        operator: Operator,
        operands: &mut VecDeque<u64>,
        total: u64,
        expected_total: u64,
    ) -> bool {
        if total > expected_total {
            return false;
        }
        match operands.pop_front() {
            Some(n) => {
                let add_valid = match operator {
                    Operator::Add => is_valid(
                        Operator::Add,
                        &mut operands.clone(),
                        total + n,
                        expected_total,
                    ),
                    Operator::Multiply => is_valid(
                        Operator::Add,
                        &mut operands.clone(),
                        total + n,
                        expected_total,
                    ),
                };
                let multiply_valid = match operator {
                    Operator::Add => is_valid(
                        Operator::Multiply,
                        &mut operands.clone(),
                        total * n,
                        expected_total,
                    ),
                    Operator::Multiply => is_valid(
                        Operator::Multiply,
                        &mut operands.clone(),
                        total * n,
                        expected_total,
                    ),
                };
                add_valid || multiply_valid
            }
            None => total == expected_total,
        }
    }
    let mut res = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let mut split = line.split(":");
        let expected_total = split.next().unwrap().parse::<u64>().unwrap();
        let mut operands = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        let first_operand = operands.pop_front().unwrap();
        let is_valid = is_valid(
            Operator::Add,
            &mut operands.clone(),
            first_operand,
            expected_total,
        ) || is_valid(
            Operator::Multiply,
            &mut operands.clone(),
            first_operand,
            expected_total,
        );
        if is_valid {
            res += expected_total;
            res2 += 1
        }
    }
    println!("{}", res2);
    res
}

#[allow(dead_code)]
fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749)
    }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 0)
    // }
    //
    #[test]
    fn real_part1() {
        println!("Part 1 Output: {}", part1(REAL_INPUT))
    }

    // #[test]
    // fn real_part2() {
    //     println!("Part 2 Output: {}", part2(REAL_INPUT))
    // }
}
