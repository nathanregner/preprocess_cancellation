#![feature(assert_matches)]

mod error;
mod generator;
mod model;
mod parser;
mod py;
mod slicers;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use crate::generator::rewrite_to_string;
pub use crate::slicers::Slicer;
use generator::rewrite;
use py::{FileIter, FileLike, GCodeError};
use pyo3::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[pymodule]
fn preprocess_cancellation(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(preprocess_slicer, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_cura, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_ideamaker, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_m486, m)?)?;
    m.add("GCodeError", py.get_type::<GCodeError>())?;
    Ok(())
}

#[pyfunction]
pub fn preprocess_slicer(file_like: FileLike) -> PyResult<FileIter> {
    let mut src = BufReader::new(File::open(&file_like)?);
    let mut objects = slicers::slic3r::list_objects(&mut src)?;

    Ok(match rewrite(file_like.as_ref(), &mut objects)? {
        None => FileIter(None),
        Some(dst) => FileIter(Some(BufReader::new(dst))),
    })
}

#[pyfunction]
pub fn preprocess_cura(file_like: FileLike) -> PyResult<()> {
    let mut src = BufReader::new(File::open(&file_like)?);
    let mut objects = slicers::cura::list_objects(&mut src)?;
    if objects.is_empty() {
        println!("preprocess_slicer: no objects found");
        return Ok(());
    }
    rewrite(file_like.as_ref(), &mut objects).unwrap();
    Ok(())
}

#[pyfunction]
pub fn preprocess_ideamaker(_file_path: FileLike) -> PyResult<()> {
    todo!()
}

#[pyfunction]
pub fn preprocess_m486(_file_path: FileLike) -> PyResult<()> {
    todo!()
}
