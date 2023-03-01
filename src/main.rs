mod file_parser;

use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{fs, io, process};

use clap::Parser;

use crate::file_parser::FileParser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    search_word: Option<String>,
    search_target: Option<String>,

    /// Print num lines of trailing context after each match.
    #[arg(short = 'A', long, default_value_t = 0)]
    after_context: u8,

    /// Print num lines of leading context before each match.
    #[arg(short = 'B', long, default_value_t = 0)]
    before_context: u8,

    /// Print num lines of leading and trailing context surrounding each match.
    #[arg(short = 'C', long, default_value_t = 0)]
    context: u8,
}

struct TargetDir {
    root: Box<dyn Iterator<Item = io::Result<DirEntry>>>,
    children: Box<dyn Iterator<Item = TargetDir>>,
}

impl TargetDir {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let root = Box::new(fs::read_dir(&path)?);
        let children = Box::new(fs::read_dir(&path)?.filter_map(|e| {
            let e = e.ok()?;
            if e.file_type().ok()?.is_dir() {
                return TargetDir::new(e.path()).ok();
            }
            None
        }));
        Ok(TargetDir { root, children })
    }

    fn entries(self) -> Box<dyn Iterator<Item = io::Result<DirEntry>>> {
        Box::new(self.root.chain(self.children.flat_map(|s| s.entries())))
    }
}

impl Iterator for TargetDir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.root.next() {
            return Some(item);
        }
        if let Some(child) = self.children.next() {
            self.root = child.entries();
            return self.next();
        }
        None
    }
}

fn search_file(
    file: PathBuf,
    word: &str,
    before_context: u8,
    after_context: u8,
    context: u8,
) -> io::Result<Vec<FileParser>> {
    let f = File::open(&*file).expect("file not found");
    let reader = BufReader::new(f);
    let mut file_parsers = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(word) {
            let file_parser = FileParser::new(
                file.to_str().unwrap().to_string(),
                index,
                before_context,
                after_context,
                context,
            );
            file_parsers.push(file_parser);
        }
    }
    Ok(file_parsers)
}

fn main() {
    let cli = Cli::parse();

    let search_word = cli
        .search_word
        .as_deref()
        .expect("'search_word｀ is an invalid argument.");
    let search_target = cli
        .search_target
        .as_deref()
        .expect("'search_word｀ is an invalid argument.");

    let after_context = cli.after_context;
    let before_context = cli.before_context;
    let context = cli.context;
    let mut search_result;

    if Path::new(search_target).is_file() {
        let search_target = PathBuf::from(search_target);
        search_result = search_file(
            search_target,
            search_word,
            before_context,
            after_context,
            context,
        );

        for file_parser in search_result.unwrap() {
            file_parser.parse(before_context, after_context, context);
        }
    } else if Path::new(search_target).is_dir() {
        let files = TargetDir::new(search_target)
            .unwrap()
            .filter_map(|entry| Some(entry.ok()?.path()))
            .collect::<Vec<_>>();
        for file in files {
            search_result = search_file(file, search_word, before_context, after_context, context);

            for file_parser in search_result.unwrap() {
                file_parser.parse(before_context, after_context, context);
            }
        }
    } else {
        eprintln!("search target `{}` is not correct!", search_target);
        process::exit(1);
    }
}
