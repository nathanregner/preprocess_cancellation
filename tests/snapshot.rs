use preprocess_cancellation::{Result, Slicer};
use rstest::rstest;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::PathBuf;

fn process(slicer: Slicer, mut src: BufReader<File>) -> Result<String> {
    let patch = slicer.format_patch(&mut src)?;
    src.seek(SeekFrom::Start(0))?;
    let mut buf = vec![];
    patch.apply(src, &mut buf)?;
    Ok(String::from_utf8(buf)?)
}

fn patch_infer(file: File) -> Result<String> {
    let mut src = BufReader::new(file);
    let slicer = Slicer::infer(&mut src)?;
    process(slicer, src)
}

#[rstest]
fn snapshot(#[files("./GCode/*.gcode")] path: PathBuf) {
    let file = File::open(&path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    insta::assert_display_snapshot!(file_name, patch_infer(file).unwrap());
}

#[rstest]
fn m486(#[files("./GCode/m486/*.gcode")] path: PathBuf) {
    let file = File::open(&path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    insta::assert_display_snapshot!(
        file_name,
        process(Slicer::M486, BufReader::new(file)).unwrap()
    );
}
