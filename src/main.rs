use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::{env, fs, io, process};

struct TargetDir {
    root: Box<dyn Iterator<Item = io::Result<DirEntry>>>,
    children: Box<dyn Iterator<Item = TargetDir>>,
}

impl TargetDir {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let root = Box::new(fs::read_dir(&path)?);
        let children = Box::new(
            fs::read_dir(&path)?
                .filter_map(|e| {
                    let e = e.ok()?;
                    if e.file_type().ok()?.is_dir() {
                        return Some(TargetDir::new(e.path()).ok()?);
                    }
                    None
                })
        );
        Ok(TargetDir { root, children })
    }

    fn entries(self) -> Box<dyn Iterator<Item = io::Result<DirEntry>>> {
        Box::new(
            self.root.chain(self.children.map(|s| s.entries()).flatten()),
        )
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let search_txt = &args[1];
    let search_target = &args[2];

    println!("{}", search_target);

    if Path::new(search_target).is_file() {
        let f = File::open(search_target).expect("file not found");
        let reader = BufReader::new(f);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains(search_txt) {
                println!("{}: {}", index, line);
            }
        }
        Ok(())
    } else if Path::new(search_target).is_dir() {
        let files = TargetDir::new(search_target)?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .collect::<Vec<_>>();
        for file in files {
            let f = File::open(file).expect("file not found");
            let reader = BufReader::new(f);
            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                if line.contains(search_txt) {
                    println!("{}: {}", index, line);
                }
            }
        }
        Ok(())
    } else {
        eprintln!("search target `{}` is not correct!", search_target);
        process::exit(1);
    }
}
