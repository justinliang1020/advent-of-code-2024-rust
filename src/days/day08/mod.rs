use std::collections::HashMap;

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let grid = {
        let mut g: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            g.push(line.chars().collect());
        }
        g
    };

    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '.' {
                continue;
            }
            antenna_map
                .entry(grid[r][c])
                .and_modify(|v| v.push((r, c)))
                .or_insert(vec![(r, c)]);
        }
    }

    let mut antinode_grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    fn is_inside(r: i32, c: i32, grid: &[Vec<char>]) -> bool {
        r >= 0 && r < (grid.len() as i32) && c >= 0 && c < (grid[0].len() as i32)
    }

    for (_, v) in antenna_map {
        for i1 in 0..v.len() {
            for i2 in i1 + 1..v.len() {
                let (r1, c1) = (v[i1].0 as i32, v[i1].1 as i32);
                let (r2, c2) = (v[i2].0 as i32, v[i2].1 as i32);

                let (rv, cv) = (r2 - r1, c2 - c1);

                let (ra1, ca1) = (r1 - rv, c1 - cv);
                let (ra2, ca2) = (r2 + rv, c2 + cv);

                if is_inside(ra1, ca1, &grid) {
                    antinode_grid[ra1 as usize][ca1 as usize] = true;
                }

                if is_inside(ra2, ca2, &grid) {
                    antinode_grid[ra2 as usize][ca2 as usize] = true;
                }
            }
        }
    }

    antinode_grid
        .iter()
        .map(|v| v.iter().filter(|&&b| b).count() as u32)
        .sum()
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let grid = {
        let mut g: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            g.push(line.chars().collect());
        }
        g
    };

    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '.' {
                continue;
            }
            antenna_map
                .entry(grid[r][c])
                .and_modify(|v| v.push((r, c)))
                .or_insert(vec![(r, c)]);
        }
    }

    let mut antinode_grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    fn is_inside(r: i32, c: i32, grid: &[Vec<char>]) -> bool {
        r >= 0 && r < (grid.len() as i32) && c >= 0 && c < (grid[0].len() as i32)
    }

    for (_, v) in antenna_map {
        for i1 in 0..v.len() {
            for i2 in i1 + 1..v.len() {
                let (r1, c1) = (v[i1].0 as i32, v[i1].1 as i32);
                let (r2, c2) = (v[i2].0 as i32, v[i2].1 as i32);

                let (rv, cv) = (r2 - r1, c2 - c1);

                let mut i = 1;
                loop {
                    let (ra1, ca1) = (r1 - rv * i, c1 - cv * i);
                    if is_inside(ra1, ca1, &grid) {
                        antinode_grid[ra1 as usize][ca1 as usize] = true;
                    } else {
                        break;
                    }
                    i += 1
                }

                let mut i = 1;
                loop {
                    let (ra2, ca2) = (r2 + rv * i, c2 + cv * i);
                    if is_inside(ra2, ca2, &grid) {
                        antinode_grid[ra2 as usize][ca2 as usize] = true;
                    } else {
                        break;
                    }
                    i += 1
                }

                antinode_grid[r1 as usize][c1 as usize] = true;
                antinode_grid[r2 as usize][c2 as usize] = true;
            }
        }
    }

    // for line in &antinode_grid {
    //     println!("{:?}", line.iter().filter(|&&b| b).count() as u32);
    //     println!("{:?}", line);
    // }

    antinode_grid
        .iter()
        .map(|v| v.iter().filter(|&&b| b).count() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("input.txt");
    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 14)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 34)
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
