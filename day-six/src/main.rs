use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let data = read_file(String::from("./day-six/input/data.txt"));
    let mut guard = Guard::parse_data(data);

    let mut locations: HashSet<usize> = HashSet::new();
    while let Some(location) = guard.next_step() {
        let _ = locations.insert(location);
    }

    let location_count = locations.len();
    println!("The guards patrol path has {location_count} unique locations");
    println!(
        "There are {0:?} locations where a new obstruction would cause a loop",
        guard.loop_locations.len()
    );
    println!("Merry Christmas");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

type Cords = (usize, usize);

struct Guard {
    pub data: Box<[u8]>,
    pub data_width: usize,
    pub data_height: usize,
    actual_width: usize,
    pub pos: Cords,
    pub dir: Direction,
    pub patrol_path: HashMap<usize, Direction>,
    pub loop_locations: Vec<Cords>,
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

        let mut patrol_path = HashMap::new();
        patrol_path.insert(index(pos, actual_width), Direction::Up);
        Self {
            data,
            data_width,
            data_height,
            actual_width,
            dir: Direction::Up,
            pos,
            patrol_path,
            loop_locations: Vec::new(),
        }
    }

    pub fn get_position(&self, cords: Cords) -> &u8 {
        self.data
            .get(index(cords, self.actual_width))
            .expect("INDEX OUT OF BOUNDS")
    }

    pub fn turn(&mut self) {
        self.dir = self.dir.turn_right();
    }

    pub fn next(&self, dir: &Direction, pos: Cords) -> Option<Cords> {
        let (x, y) = pos;
        match dir {
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
        }
    }

    pub fn next_step(&mut self) -> Option<usize> {
        let next = self.next(&self.dir, self.pos);
        if let Some(next) = next {
            let next_pos = self.get_position(next);
            if is_obstructed(next_pos) {
                self.turn();
                self.next_step()
            } else {
                //println!("{0:?}", self.patrol_path);
                if b'^' != *next_pos
                    && !self
                        .patrol_path
                        .contains_key(&index(next, self.actual_width))
                    && self.check_loop(self.dir.turn_right(), self.pos)
                {
                    self.loop_locations.push(next);
                }
                self.patrol_path
                    .insert(index(next, self.actual_width), self.dir.clone());
                self.pos = next;
                Some(index(next, self.actual_width))
            }
        } else {
            None
        }
    }

    pub fn check_loop(&self, mut dir: Direction, mut pos: Cords) -> bool {
        let mut loop_path: HashMap<usize, Direction> = self.patrol_path.clone();

        while let Some(next) = self.next(&dir, pos) {
            let next_pos = self.get_position(next);
            if is_obstructed(next_pos) {
                dir = dir.turn_right()
            } else {
                if let Some(loop_dir) = loop_path.get(&index(next, self.actual_width)) {
                    //println!("Crossing paths Postion: {pos:?}, Last dir: {loop_dir:?}, Current dir: {dir:?}");
                    if *loop_dir == dir {
                        return true;
                    }
                }
                loop_path.insert(index(next, self.actual_width), dir.clone());
                pos = next;
            }
        }
        //println!("{loop_path:?}");
        false
    }
}

fn read_file(file_path: String) -> Vec<u8> {
    let file_path = Path::new(&file_path);
    let data: Vec<u8> = fs::read(file_path).unwrap();
    data
}

#[inline]
fn is_obstructed(pos_value: &u8) -> bool {
    !(b'.' == *pos_value || b'^' == *pos_value)
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
        let next = guard.next_step();
        assert!(next.is_some());
        assert_eq!(59, next.expect("Previously Asserted"));
    }

    #[test]
    fn test_next_turn() {
        let mut guard = Guard::parse_data(example_data());
        guard.pos = (4, 1);
        let next = guard.next_step();
        assert!(next.is_some());
        assert_eq!(16, next.expect("Previously Asserted"));
        guard.pos = (8, 1);
        let next = guard.next_step();
        assert!(next.is_some());
        assert_eq!(30, next.expect("Previously Asserted"));
    }

    #[test]
    fn test_walk_path() {
        let mut guard = Guard::parse_data(example_data());
        let mut locations: HashSet<usize> = HashSet::new();

        while let Some(location) = guard.next_step() {
            let _ = locations.insert(location);
        }
        assert_eq!(41, locations.len());
    }

    #[test]
    fn test_loop_locations() {
        let mut guard = Guard::parse_data(example_data());
        let mut locations: HashSet<usize> = HashSet::new();

        while let Some(location) = guard.next_step() {
            let _ = locations.insert(location);
        }
        println!("{0:?}", guard.loop_locations);
        for location in &guard.loop_locations {
            guard.data[index(*location, guard.actual_width)] = b'O';
        }
        println!("{}", String::from_utf8(guard.data.to_vec()).unwrap());

        assert_eq!(6, guard.loop_locations.len());
        assert!(guard.loop_locations.contains(&(3, 6)));
        assert!(guard.loop_locations.contains(&(6, 7)));
        assert!(guard.loop_locations.contains(&(7, 7)));
        assert!(guard.loop_locations.contains(&(1, 8)));
        assert!(guard.loop_locations.contains(&(3, 8)));
        assert!(guard.loop_locations.contains(&(7, 9)));
    }

    #[test]
    fn test_obstructed_loop() {
        let mut guard = Guard::parse_data(example_data_obstructed_loop());
        let mut locations: HashSet<usize> = HashSet::new();

        while let Some(location) = guard.next_step() {
            let _ = locations.insert(location);
        }
        println!("{0:?}", guard.loop_locations);
        for location in &guard.loop_locations {
            guard.data[index(*location, guard.actual_width)] = b'O';
        }
        println!("{}", String::from_utf8(guard.data.to_vec()).unwrap());

        assert_eq!(7, guard.loop_locations.len());

        assert!(guard.loop_locations.contains(&(3, 6)));
        assert!(guard.loop_locations.contains(&(6, 7)));
        assert!(guard.loop_locations.contains(&(7, 7)));
        assert!(guard.loop_locations.contains(&(1, 8)));
        assert!(guard.loop_locations.contains(&(3, 8)));
        assert!(guard.loop_locations.contains(&(7, 9)));

        assert!(guard.loop_locations.contains(&(4, 8)));
    }

    #[test]
    fn test_reddit_debug_map() {
        let mut guard = Guard::parse_data(debug_data());
        let mut locations: HashSet<usize> = HashSet::new();

        while let Some(location) = guard.next_step() {
            let _ = locations.insert(location);
        }

        println!("{0:?}", guard.loop_locations);
        for location in &guard.loop_locations {
            guard.data[index(*location, guard.actual_width)] = b'O';
        }
        println!("{}", String::from_utf8(guard.data.to_vec()).unwrap());

        assert_eq!(15, guard.loop_locations.len());
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

    fn example_data_obstructed_loop() -> Vec<u8> {
        b"....##....
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

    fn debug_data() -> Vec<u8> {
        b"...........#.....#......
...................#....
...#.....##.............
......................#.
..................#.....
..#.....................
....................#...
........................
.#........^.............
..........#..........#..
..#.....#..........#....
........#.....#..#......"
            .into()
    }
}
