use std::fs;
use std::path::Path;

fn main() {
    let data = read_file(String::from("./day-two/input/reports.txt"));
    let mut data = ReportData::new(data);

    let mut safe_reports: Vec<Vec<usize>> = Vec::new();
    let mut unsafe_reports: Vec<Vec<usize>> = Vec::new();

    while let Some(report) = data.read_report() {
        if is_report_safe(&report) {
            safe_reports.push(report);
        } else {
            unsafe_reports.push(report);
        }
    }

    println!(
        "There are {} safe reports and {} unsafe reports.",
        safe_reports.len(),
        unsafe_reports.len()
    );
}

#[derive(Debug)]
struct DamperedBool {
    val: bool,
    pub damper: bool,
}

impl DamperedBool {
    pub fn new() -> Self {
        DamperedBool {
            val: true,
            damper: true,
        }
    }

    pub fn set_false(&mut self) -> bool {
        if self.damper {
            self.damper = false;
            false
        } else {
            self.val = false;
            true
        }
    }

    pub fn get_val(&self) -> bool {
        self.val
    }
}

fn is_report_safe(report: &Vec<usize>) -> bool {
    let log_str = format!("is_report_safe({:?})", &report);
    let mut report = report.iter();
    let mut safe = DamperedBool::new();
    let mut is_accending: Option<bool> = None;
    let mut last_val = *report.next().unwrap();

    let mut trace_log_str = String::from("");

    for val in report {
        let diff: isize = (*val as isize) - (last_val as isize);
        if !(-4 < diff && diff < 4 && diff != 0) {
            if !safe.set_false() {
                continue;
            }
        } else if let Some(accending) = is_accending {
            if accending && 0 > diff {
                if !safe.set_false() {
                    continue;
                }
            } else if !accending && 0 < diff && !safe.set_false() {
                continue;
            }
        } else {
            is_accending = Some(*val > last_val);
        }

        trace_log_str = format!("{trace_log_str}\nLast Val: {last_val} | Val: {val} | Accending: {is_accending:?} | Diff: {diff} | Safe: {safe:?}");
        last_val = *val;

        if !safe.get_val() {
            break;
        }
    }
    println!("{trace_log_str}");
    println!("{log_str} -> {safe:?}");
    safe.get_val()
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
            .reduce(|acc, val| acc + val)
    }

    fn read_report(&mut self) -> Option<Vec<usize>> {
        let mut report: Vec<usize> = Vec::new();

        while let Some(val) = self.read_value() {
            report.push(val);
        }

        if self.peek().is_some() {
            let _ = self.read_byte();
            Some(report)
        } else if report.is_empty() {
            None
        } else {
            Some(report)
        }
    }
}

fn read_file(file_path: String) -> Vec<u8> {
    let file_path = Path::new(&file_path);
    let data: Vec<u8> = fs::read(file_path).unwrap();
    data
}

#[cfg(test)]
mod day_two {
    use super::{is_report_safe, ReportData};

    #[test]
    fn report_data_peek() {
        let data = setup_data();
        let report_data = ReportData::new(data);
        let peek = report_data.peek();
        assert!(peek.is_some());
        assert_eq!(b'7', *peek.unwrap());
        assert_eq!(0, report_data.cursor);
    }

    #[test]
    fn report_data_read_byte() {
        let data = setup_data();
        let mut report_data = ReportData::new(data);
        let byte = report_data.read_byte();

        assert!(byte.is_some());
        assert_eq!(b'7', byte.unwrap());
        assert_eq!(1, report_data.cursor);
    }

    #[test]
    fn report_data_skip_space() {
        let data = setup_data();
        let mut report_data = ReportData::new(data);
        report_data.cursor = 1;
        report_data.skip_space();
        assert_eq!(2, report_data.cursor);
        assert_eq!(b'6', *report_data.peek().unwrap());

        report_data.skip_space();
        assert_eq!(2, report_data.cursor);
        assert_eq!(b'6', *report_data.peek().unwrap());
    }

    #[test]
    fn report_data_read_value() {
        let data = setup_data();
        let mut report_data = ReportData::new(data);

        assert_eq!(Some(7), report_data.read_value());
        assert_eq!(Some(6), report_data.read_value());
        assert_eq!(Some(42), report_data.read_value());
        assert_eq!(Some(2), report_data.read_value());
        assert_eq!(Some(1), report_data.read_value());
        assert_eq!(None, report_data.read_value());
    }

    #[test]
    fn report_data_read_report() {
        let data = setup_data();
        let mut report_data = ReportData::new(data);

        assert_eq!(Some(vec![7, 6, 42, 2, 1,]), report_data.read_report());
        assert_eq!(Some(vec![1, 2, 7, 8, 9]), report_data.read_report());
        assert_eq!(Some(vec![9, 7, 6, 2, 1]), report_data.read_report());
        assert_eq!(Some(vec![1, 3, 2, 4, 5]), report_data.read_report());
        assert_eq!(Some(vec![8, 6, 4, 4, 1]), report_data.read_report());
        assert_eq!(Some(vec![1, 3, 6, 7, 9]), report_data.read_report());
        assert!(report_data.read_report().is_none());
        assert!(report_data.read_report().is_none());
    }

    #[test]
    fn is_report_safe_example_one() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_example_two() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_example_three() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_example_four() {
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_example_five() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_example_six() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_decending_safe() {
        let mut report = vec![7, 6, 4, 2, 1];
        report.reverse();
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_accending_damper_safe() {
        let report = vec![10, 6, 7, 5, 2];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_accending_unsafe() {
        let report = vec![10, 6, 7, 2, 1];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_inconistent_unsafe() {
        let report = vec![10, 14, 15, 11];
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_no_difference_unsafe() {
        let report = vec![10, 11, 11, 12];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_duplicates_first() {
        let report = vec![14, 14, 11, 10, 7, 5, 2];
        assert!(is_report_safe(&report));
    }

    #[test]
    fn is_report_safe_bad_data() {
        let report = vec![86, 87, 85, 87, 89, 92, 95];
        assert!(!is_report_safe(&report));
    }

    #[test]
    #[ignore = "This day is not finished, I'll come back to it later"]
    fn is_report_safe_bad_data_two() {
        let report = vec![75, 78, 79, 82, 85, 85];
        assert!(!is_report_safe(&report));
    }

    fn setup_data() -> Vec<u8> {
        let data = br"7 6 42 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        Vec::from(data)
    }
}
