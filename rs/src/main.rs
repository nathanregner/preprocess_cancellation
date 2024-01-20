use std::{env, fs::File};

use preprocess_cancellation::extract_objects;

fn main() {
    let mut args = env::args();
    args.next();
    let file_name = args.next().expect("file name");
    println!("processing file: {}", file_name);
    let mut file = File::open(file_name).expect("readable file");
    extract_objects(&mut file).unwrap()
}
