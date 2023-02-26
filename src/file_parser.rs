use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::Range;

pub struct FileParser {
    file_name: String,
    line_num: usize,
    before_context_range: Range<usize>,
    after_context_range: Range<usize>,
}

impl FileParser {
    pub fn new(file_name: String, line_num: usize, before_context: u8, after_context: u8) -> Self {
        let file_name = file_name;
        let line_num = line_num;
        let before_context = usize::from(before_context);
        let before_context_range = line_num - before_context..line_num;
        let after_context = usize::from(after_context);
        let after_context_range = line_num + 1..line_num + after_context + 1;

        FileParser {
            file_name,
            line_num,
            before_context_range,
            after_context_range,
        }
    }

    pub fn parse(self, before_context: u8, after_context: u8, context: u8) {
        let file = File::open(&self.file_name).unwrap();

        let reader = BufReader::new(file);
        let context = usize::from(context);

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            if self.before_context_range.contains(&i) {
                println!("{}-{}-{}", self.file_name, i, line)
            }

            if i == self.line_num {
                println!("{}:{}:{}", self.file_name, i, line);
            }

            if self.after_context_range.contains(&i) {
                println!("{}-{}-{}", self.file_name, i, line)
            }
        }
    }
}
