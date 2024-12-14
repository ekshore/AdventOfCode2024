fn main() {
    println!("Merry Christmas");
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Cords = (usize, usize);

struct Guard {
    pub data: Box<[u8]>,
    pub data_width: usize,
    pub data_height: usize,
    actual_width: usize,
    pub pos: Cords,
    pub dir: Direction,
}

impl Guard {
    pub fn parse_data(raw_data: Vec<u8>) -> Self {
        let data_width = raw_data
            .iter()
            .enumerate()
            .find(|(_, &e)| e == b'\n')
            .map_or_else(|| 0, |(idx, _)| idx);
        let actual_width = data_width + 1;
        let data_height = (raw_data.len() + 1) / actual_width;

        let pos_idx = raw_data
            .iter()
            .enumerate()
            .find(|(_, &e)| e == b'^')
            .map_or_else(|| 0, |(idx, _)| idx);
        let pos = (pos_idx % actual_width, pos_idx / actual_width);

        let data: Box<[u8]> = raw_data.into();
        Self {
            data,
            data_width,
            data_height,
            actual_width,
            dir: Direction::Up,
            pos,
        }
    }

    pub fn get_position(&self, cords: Cords) -> &u8 {
        self.data
            .get(index(cords, self.actual_width))
            .expect("INDEX OUT OF BOUNDS")
    }

    pub fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn next(&mut self) -> Option<usize> {
        let (x, y) = self.pos;
        let next = match self.dir {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::Down => {
                if y == self.data_height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::Right => {
                if x == self.data_width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        };

        if let Some(next) = next {
            let next_pos = self.get_position(next);
            if !(b'.' == *next_pos || b'^' == *next_pos) {
                self.turn();
                self.next()
            } else {
                self.pos = next;
                Some(index(next, self.actual_width))
            }
        } else {
            None
        }
    }
}

#[inline]
fn index(cords: Cords, offset_width: usize) -> usize {
    let (x, y) = cords;
    y * offset_width + x
}

#[cfg(test)]
mod day_six {
    use super::*;

    #[test]
    fn example_one_data_load() {
        let guard = Guard::parse_data(example_data());
        assert_eq!(10, guard.data_width);
        assert_eq!(10, guard.data_height);
    }

    #[test]
    fn example_one_data_get_byte_last() {
        let guard = Guard::parse_data(example_data());
        assert_eq!(b'.', *guard.get_position((9, 9)));
    }

    #[test]
    fn test_index() {
        let guard = Guard::parse_data(example_data());
        assert_eq!(70, index(guard.pos, guard.actual_width));
    }

    #[test]
    fn test_next() {
        let mut guard = Guard::parse_data(example_data());
        let next = guard.next();
        assert!(next.is_some());
        assert_eq!(59, next.expect("Previously Asserted"));
    }

    #[test]
    fn test_next_turn() {
        let mut guard = Guard::parse_data(example_data());
        guard.pos = (4, 1);
        let next = guard.next();
        assert!(next.is_some());
        assert_eq!(16, next.expect("Previously Asserted"));
        guard.pos = (8, 1);
        let next = guard.next();
        assert!(next.is_some());
        assert_eq!(30, next.expect("Previously Asserted"));
    }

    fn example_data() -> Vec<u8> {
        b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .into()
    }
}
