fn main() {
    println!("Merry Christmas");
}

struct WordSearch {
    pub data: Box<[u8]>,
    pub data_width: usize,
    pub data_height: usize,
    actual_width: usize,
}

impl WordSearch {
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
        self.data.get(y * self.actual_width + x).expect("INDEX OUT OF BOUNDS")
    }
}

#[cfg(test)]
mod day_four {
    use super::*;

    #[test]
    fn example_one_data_load() {
        let word_search = WordSearch::parse_data(example_one_data());
        assert_eq!(6, word_search.data_width);
        assert_eq!(5, word_search.data_height);
        assert_eq!(*b"..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X...Z", *word_search.data);
    }

    #[test]
    fn example_two_data_load() {
        let word_search = WordSearch::parse_data(example_two_data());
        assert_eq!(10, word_search.data_width);
        assert_eq!(10, word_search.data_height);
    }

    #[test]
    fn example_one_data_get_byte_last() {
        let data = WordSearch::parse_data(example_one_data());
        assert_eq!(b'Z', *data.get_byte(5, 4));
    }

    #[test]
    fn example_one_data_get_byte_first_s() {
        let data = WordSearch::parse_data(example_one_data());
        assert_eq!(b'S', *data.get_byte(1,1));
    }

    fn example_one_data() -> Vec<u8> {
        let data = b"..X...
.SAMX.
.A..A.
XMAS.S
.X...Z";
        data.to_vec()
    }

    fn example_two_data() -> Vec<u8> {
        let data = b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        data.to_vec()
    }
}
