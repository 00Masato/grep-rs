use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use crate::search_file;
use colored::*;

pub struct FileParser {
    file_name: String,
    line_num: usize,
    search_word: String,
    before_context_range: Range<usize>,
    after_context_range: Range<usize>,
    context_range: (Range<usize>, Range<usize>),
}

impl FileParser {
    pub fn new(
        file_name: String,
        line_num: usize,
        search_word: String,
        before_context: u8,
        after_context: u8,
        context: u8,
    ) -> Self {
        let file_name = file_name;
        let line_num = line_num;
        let search_word = search_word;
        let before_context = usize::from(before_context);
        let before_context_range = line_num - before_context..line_num;
        let after_context = usize::from(after_context);
        let after_context_range = line_num + 1..line_num + after_context + 1;
        let context = usize::from(context);
        let context_range = (
            line_num - context..line_num,
            line_num + 1..line_num + context + 1,
        );

        FileParser {
            file_name,
            line_num,
            search_word,
            before_context_range,
            after_context_range,
            context_range,
        }
    }

    pub fn parse(self) {
        let file = File::open(&self.file_name).unwrap();

        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line_num = (i + 1).to_string().green();

            if self.before_context_range.contains(&i) || self.context_range.0.contains(&i) {
                println!("{}-{}-{}", self.file_name, line_num, line)
            }

            if i == self.line_num {
                // TODO: When there are more than two in a row, both should be colored.
                let divided_line_list = line.split(&self.search_word).collect::<Vec<_>>();
                println!(
                    "{}:{}:{}{}{}",
                    self.file_name,
                    line_num,
                    divided_line_list[0],
                    self.search_word.red(),
                    divided_line_list[1]
                );
            }

            if self.after_context_range.contains(&i) || self.context_range.1.contains(&i) {
                println!("{}-{}-{}", self.file_name, line_num, line)
            }
        }
    }
}
