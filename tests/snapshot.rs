use preprocess_cancellation::{Result, Slicer};
use rstest::rstest;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::PathBuf;

fn process_example(file: File) -> Result<String> {
    let mut src = BufReader::new(file);
    let patch = Slicer::infer(&mut src)?.format_patch(&mut src)?;
    src.seek(SeekFrom::Start(0))?;
    let mut buf = vec![];
    patch.apply(src, &mut buf)?;
    Ok(String::from_utf8(buf)?)
}

#[rstest]
fn snapshot(#[files("./GCode/*.gcode")] path: PathBuf) {
    let file = File::open(&path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    insta::assert_display_snapshot!(file_name, process_example(file).unwrap());
}
