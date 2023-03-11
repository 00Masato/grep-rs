use std::io::Read;
use std::process::{Command, Stdio};

const TEXT: &'static str = "\
tests/data/test_data.txt:6:How dreary to be somebody!
tests/data/test_data.txt:7:How public, like a frog
";

const FAILED_MESSAGE: &'static str = "\
search target `tests/data/failed_data.txt` is not correct!
";

// success
#[test]
fn success_text_test() {
    let child = Command::new("target/debug/grep-rs")
        .args(&["How", "tests/data/test_data.txt"])
        .stdout(Stdio::piped())
        .spawn();

    let mut stdout = child.unwrap().stdout.take().unwrap();
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).unwrap();

    assert_eq!(TEXT, buffer);
}

// failed(target_file not found)
#[test]
fn failed_text_test() {
    let child = Command::new("target/debug/grep-rs")
        .args(&["How", "tests/data/failed_data.txt"])
        .stderr(Stdio::piped())
        .spawn();

    let mut stderr = child.unwrap().stderr.take().unwrap();
    let mut buffer = String::new();
    stderr.read_to_string(&mut buffer).unwrap();

    assert_eq!(FAILED_MESSAGE, buffer);
}
