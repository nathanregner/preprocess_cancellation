use preprocess_cancellation::Slicer;
use std::io::BufReader;

fn main() {
    let file_path = std::env::args().nth(1).expect("missing file path");
    // print the file
    let file = std::fs::File::open(file_path).expect("failed to open file");

    dbg!(Slicer::Slic3r
        .list_objects(&mut BufReader::new(file))
        .unwrap());
}
