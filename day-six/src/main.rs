fn main() {
    println!("Merry Christmas");
}

type Cords = (usize, usize);

struct Guard {
    pub data: Box<[u8]>,
    pub data_width: usize,
    pub data_height: usize,
    actual_width: usize,
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
        let data: Box<[u8]> = raw_data.into();
        Self {
            data,
            data_width,
            data_height,
            actual_width,
        }
    }

    pub fn get_byte(&self, x: usize, y: usize) -> &u8 {
        self.data
            .get(y * self.actual_width + x)
            .expect("INDEX OUT OF BOUNDS")
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
        assert_eq!(b'.', *guard.get_byte(9, 9));
    }

    #[test]
    fn test_index() {
        let guard = Guard::parse_data(example_data());
        assert_eq!(70, index((4, 6), guard.actual_width));
    }

    fn example_data() -> Vec<u8>{
        b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".into()
    }
}
