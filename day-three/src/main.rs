fn main() {
    println!("Merry Christmas");
}

struct ReportData {
    data: Box<[u8]>,
    pub cursor: usize,
    data_len: usize,
}

impl ReportData {
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

    fn skip_space(&mut self) {
        while let Some(b) = self.peek() {
            if *b == b' ' {
                let _ = self.read_byte();
            } else {
                break;
            }
        }
    }

    fn read_value(&mut self) -> Option<usize> {
        let mut buff: Vec<u8> = Vec::new();
        while let Some(byte) = self.peek() {
            if *byte == b' ' {
                self.skip_space();
                break;
            }
            if *byte == b'\n' {
                break;
            }
            buff.push(self.read_byte().unwrap());
        }

        buff.reverse();
        buff.into_iter()
            .enumerate()
            .map(|(idx, val)| (val - 48) as usize * (10_usize.pow((idx) as u32)))
            .reduce(|acc, val| (acc + val) as usize)
    }

    fn read_report(&mut self) -> Option<Vec<usize>> {
        let mut report: Vec<usize> = Vec::new();

        while let Some(val) = self.read_value() {
            report.push(val);
        }

        if let Some(_) = self.peek() {
            let _ = self.read_byte();
            Some(report)
        } else {
            if report.is_empty() {
                None
            } else {
                Some(report)
            }
        }
    }
}

#[cfg(test)]
mod day_three {
    let example_data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";

    #[test]
    fn example_data() {
    }
}
