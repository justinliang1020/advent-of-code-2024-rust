fn search_horizontal(grid: Vec<Vec<char>>, word: String) -> u32 {
    let reversed_word = word.chars().rev().collect::<String>();

    grid.iter()
        .map(|line| {
            let mut matches: u32 = 0;
            for end in word.len()..line.len() {
                let start = end - word.len();
                let current_word = line[start..end].iter().collect::<String>();
                if current_word == word || current_word == reversed_word {
                    matches += 1
                }
            }
            matches
        })
        .sum()
}

fn search_vertical(grid: Vec<Vec<char>>, word: String) -> u32 {
    0
}

fn search_diagonal(grid: Vec<Vec<char>>, word: String) -> u32 {
    0
}
#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word = String::from("XMAS");
    search_horizontal(grid, word)
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
        assert_eq!(part1(TEST_INPUT), 0)
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
