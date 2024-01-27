use preprocess_cancellation::{rewrite_to_string, Slicer};
use rstest::rstest;
use std::io::Cursor;

fn process_example(slicer: Slicer, input: &[u8]) -> String {
    let mut objects = slicer.list_objects(&mut Cursor::new(input)).unwrap();
    rewrite_to_string(&mut Cursor::new(input), &mut objects).expect("valid output")
}

#[rstest]
#[case(Slicer::Slic3r, include_bytes!("../GCode/superslicer.gcode"))]
#[case(Slicer::Slic3r, include_bytes!("../GCode/slic3r.gcode"))]
#[case(Slicer::Slic3r, include_bytes!("../GCode/prusaslicer.gcode"))]
#[case(Slicer::Slic3r, include_bytes!("../GCode/prusaslicer-issue1.gcode"))]
#[case(Slicer::Cura, include_bytes!("../GCode/cura.gcode"))]
fn snapshot(#[case] slicer: Slicer, #[case] input: &[u8]) {
    insta::assert_display_snapshot!(process_example(slicer, input));
}
