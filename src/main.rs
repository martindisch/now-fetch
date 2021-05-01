use std::{fs, io};

fn main() {
    let mut files = fs::read_dir("data/input/")
        .expect("Could not read input files")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Could not map entries to paths");
    files.sort();

    println!("{:?}", files);
}
