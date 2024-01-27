#![feature(assert_matches)]

mod error;
mod gcode;
mod patch;
mod py;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use self::gcode::slicers::Slicer;
use clap::builder::OsStr;
use clap::{command, value_parser, Arg, ArgAction};
use py::{FileIter, FileLike, GCodeError};
use pyo3::prelude::*;
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
    m.add_function(wrap_pyfunction!(pymain, m)?)?;
    m.add("GCodeError", py.get_type::<GCodeError>())?;
    Ok(())
}

pub fn rewrite_iter(_slicer: Slicer, file_like: FileLike) -> PyResult<FileIter> {
    let mut src = BufReader::new(File::open(file_like)?);
    let patch = Slicer::Cura.format_patch(&mut src)?;
    let dst = NamedTempFile::new()?;
    src.seek(SeekFrom::Start(0))?;
    patch.apply(src, dst.reopen()?)?;
    Ok(dst.into())
}

#[pyfunction]
pub fn preprocess_slicer(file_like: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::Slic3r, file_like)
}

#[pyfunction]
pub fn preprocess_cura(file_like: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::Cura, file_like)
}

#[pyfunction]
pub fn preprocess_ideamaker(file_like: FileLike) -> PyResult<FileIter> {
    rewrite_iter(Slicer::IdeaMaker, file_like)
}

#[pyfunction]
pub fn preprocess_m486(_file_like: FileLike) -> PyResult<()> {
    todo!()
}

#[pyfunction]
pub fn pymain() -> PyResult<()> {
    Ok(main()?)
}

pub fn main() -> crate::Result<()> {
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
                .required(false),
        )
        .get_matches();

    let output_suffix = matches.get_one::<OsStr>("output-suffix");
    let paths = matches.get_many::<PathBuf>("paths").unwrap();

    for gcode_file in paths {
        let mut src = BufReader::new(File::open(gcode_file)?);
        let temp = NamedTempFile::new()?;
        let patch = Slicer::infer(&mut src)?.format_patch(&mut src)?;
        patch.apply(src, BufWriter::new(temp.reopen()?))?;

        let dst_path = gcode_file.with_file_name({
            let mut name = gcode_file.file_name().unwrap_or_default().to_owned();
            name.push(output_suffix.unwrap_or(&OsStr::default()));
            name
        });
        std::fs::rename(temp.into_temp_path(), dst_path)?;
    }

    Ok(())
}
