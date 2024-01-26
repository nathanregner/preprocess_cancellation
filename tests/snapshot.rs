use preprocess_cancellation::{rewrite_to_string, Slicer};
use std::io::Cursor;

fn process_example(slicer: Slicer, input: &[u8]) -> String {
    let mut objects = slicer.list_objects(&mut Cursor::new(input)).unwrap();
    rewrite_to_string(&mut Cursor::new(input), &mut objects).expect("valid output")
}

#[test]
fn superslicer() {
    insta::assert_display_snapshot!(process_example(
        Slicer::Slic3r,
        &include_bytes!("../GCode/superslicer.gcode")[..]
    ));
}
