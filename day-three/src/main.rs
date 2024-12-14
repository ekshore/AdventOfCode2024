use std::{fs, path::Path};

fn main() {
    println!("Merry Christmas");
}

struct DataParser {
    data: Box<[u8]>,
    pub cursor: usize,
    data_len: usize,
}

impl DataParser {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: data.as_slice().into(),
            cursor: 0,
            data_len: data.len(),
        }
    }

    #[inline]
    fn eof(&self) -> bool {
        self.cursor > self.data_len - 1
    }

    fn peek(&self) -> Option<&u8> {
        if self.eof() {
            None
        } else {
            Some(&self.data[self.cursor])
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.eof() {
            None
        } else {
            let byte = Some(self.data[self.cursor]);
            self.cursor += 1;
            byte
        }
    }

    fn skip_invalid_chars(&mut self) {
        while let Some(b) = self.peek() {
            match *b {
                b'm' | b'u' | b'l' | b'(' | b')' | b',' | b'0'..=b'9' => {
                    return;
                }
                _ => {
                    let _ = self.read_byte();
                }
            }
        }
    }
}

fn read_file(file_path: String) -> Vec<u8> {
    let file_path = Path::new(&file_path);
    let data: Vec<u8> = fs::read(file_path).unwrap();
    data
}

#[cfg(test)]
mod day_three {
    use crate::DataParser;

    fn example_data() -> Vec<u8> {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)"
            .as_bytes()
            .to_vec()
    }

    #[test]
    fn data_parser_peek() {
        let data = example_data();
        let report_data = DataParser::new(data);
        let peek = report_data.peek();
        assert!(peek.is_some());
        assert_eq!(b'x', *peek.unwrap());
        assert_eq!(0, report_data.cursor);
    }

    #[test]
    fn data_parser_read_byte() {
        let data = example_data();
        let mut report_data = DataParser::new(data);
        let byte = report_data.read_byte();

        assert!(byte.is_some());
        assert_eq!(b'x', byte.unwrap());
        assert_eq!(1, report_data.cursor);
    }

    #[test]
    fn data_parser_skip_invalid_chars() {
        let data = example_data();
        let mut report_data = DataParser::new(data);
        report_data.skip_invalid_chars();
        assert_eq!(1, report_data.cursor);
        assert_eq!(b'm', *report_data.peek().unwrap());

        report_data.skip_invalid_chars();
        assert_eq!(1, report_data.cursor);
        assert_eq!(b'm', *report_data.peek().unwrap());

        report_data.cursor = 9;
        report_data.skip_invalid_chars();
        assert_eq!(11, report_data.cursor);
        assert_eq!(b'm', *report_data.peek().unwrap());
    }
}
