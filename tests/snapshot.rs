use preprocess_cancellation::{list_objects, rewrite_to_string};
use rstest::rstest;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn process_example(mut file: File) -> String {
    let mut objects = list_objects(&mut file).unwrap();
    rewrite_to_string(BufReader::new(file), &mut objects).expect("valid output")
}

#[rstest]
fn snapshot(#[files("./GCode/*.gcode")] path: PathBuf) {
    let file = File::open(&path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    insta::assert_display_snapshot!(file_name, process_example(file));
}
