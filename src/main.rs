use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let search_txt = &args[1];
    let filename = &args[2];

    println!("{}", filename);

    let mut f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(search_txt) {
            println!("{}: {}", index, line);
        }
    }
}
