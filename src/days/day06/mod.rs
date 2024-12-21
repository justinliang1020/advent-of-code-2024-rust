// template code for each dayXX/mod.rs file
#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(PartialEq, Debug)]
struct State {
    dir: Direction,
    pos: (usize, usize),
}

impl State {
    fn rotate_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

const GUARD: char = '^';
const INITIAL_DIRECTION: Direction = Direction::Up;
const OBSTABLE: char = '#';
const TRAVERSED: char = 'X';

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut state: State = {
        let mut r_guard = 0;
        let mut c_guard = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == GUARD {
                    r_guard = r;
                    c_guard = c;
                }
            }
        }
        State {
            dir: INITIAL_DIRECTION,
            pos: (r_guard, c_guard),
        }
    };

    loop {
        if state.pos.0 as i32 + state.dir.to_tuple().0 < 0
            || state.pos.0 as i32 + state.dir.to_tuple().0 >= grid.len() as i32
            || state.pos.1 as i32 + state.dir.to_tuple().1 < 0
            || state.pos.1 as i32 + state.dir.to_tuple().1 >= grid.len() as i32
        {
            grid[state.pos.0][state.pos.1] = TRAVERSED;
            break;
        }
        let new_pos = (
            (state.dir.to_tuple().0 + state.pos.0 as i32) as usize,
            (state.dir.to_tuple().1 + state.pos.1 as i32) as usize,
        );
        if grid[new_pos.0][new_pos.1] == OBSTABLE {
            state.rotate_right();
        } else {
            grid[state.pos.0][state.pos.1] = TRAVERSED;
            grid[new_pos.0][new_pos.1] = GUARD;
            state.pos = new_pos;
        }
    }

    {
        let mut sum = 0;
        for r in 0..grid.len() {
            for c in 0..grid.len() {
                if grid[r][c] == TRAVERSED {
                    sum += 1;
                }
            }
        }
        sum
    }
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
        assert_eq!(part1(TEST_INPUT), 41)
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
