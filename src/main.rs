use std::{env, process};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

fn main() {
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
    } else if Path::new(search_target).is_dir() {

    } else {
        eprintln!("search target `{}` is not correct!", search_target);
        process::exit(1);
    }
}
