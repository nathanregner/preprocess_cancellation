#![feature(assert_matches)]

mod error;
mod gcode;
mod patch;
mod py;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use self::gcode::slicers::Slicer;
use clap::{command, value_parser, Arg, ArgAction};
use py::{FileIter, FileLike, GCodeError};
use pyo3::prelude::*;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[pymodule]
fn preprocess_cancellation(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(preprocess_slicer, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_cura, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_ideamaker, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_m486, m)?)?;
    m.add_function(wrap_pyfunction!(_main, m)?)?;
    m.add("GCodeError", py.get_type::<GCodeError>())?;
    Ok(())
}

pub fn rewrite_iter(slicer: Slicer, src: FileLike) -> PyResult<FileIter> {
    let mut src = BufReader::new(File::open(src)?);
    let patch = slicer.format_patch(&mut src)?;
    let dst = NamedTempFile::new()?;
    src.seek(SeekFrom::Start(0))?;
    patch.apply(src, dst.reopen()?)?;
    Ok(dst.into())
}

#[pyfunction]
pub fn preprocess_slicer(src: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::Slic3r, src)
}

#[pyfunction]
pub fn preprocess_cura(src: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::Cura, src)
}

#[pyfunction]
pub fn preprocess_ideamaker(src: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::IdeaMaker, src)
}

#[pyfunction]
pub fn preprocess_m486(src: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::M486, src)
}

#[pyfunction]
pub fn _main(args: Vec<OsString>) -> PyResult<()> {
    Ok(main(args)?)
}

pub fn main<I, T>(args: I) -> crate::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = command!()
        .arg(
            Arg::new("paths")
                .action(ArgAction::Append)
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output-suffix")
                .long("output-suffix")
                .short('o')
                .required(false)
                .help(
                    "Add a suffix to gcoode output. Without this, gcode will be rewritten in place",
                ),
        )
        .arg(
            Arg::new("disable-shapely")
                .long("disable-shapely")
                .help("Deprecated, has no effect")
                .hide(true)
                .required(false)
                .num_args(0),
        )
        .get_matches_from(args);

    let output_suffix = matches.get_one::<String>("output-suffix");
    let paths = matches.get_many::<PathBuf>("paths").unwrap();

    for gcode_file in paths {
        let mut src = BufReader::new(File::open(gcode_file)?);
        let temp = NamedTempFile::new()?;
        let patch = Slicer::infer(&mut src)?.format_patch(&mut src)?;
        patch.apply(src, BufWriter::new(temp.reopen()?))?;

        let dst_path = gcode_file.with_file_name({
            let ext = gcode_file.extension().unwrap_or_default();
            let mut name = gcode_file
                .with_extension("")
                .file_name()
                .unwrap_or_default()
                .to_owned();
            name.push(OsString::from(output_suffix.unwrap_or(&String::default())));
            name.push(".");
            name.push(ext);
            name
        });
        std::fs::copy(temp, dst_path)?;
    }

    Ok(())
}
