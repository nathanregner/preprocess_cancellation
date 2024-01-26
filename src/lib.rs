mod bounding_box;
mod pyiter;
mod slicers;

use crate::slicers::rewrite;
pub use crate::slicers::{rewrite_to_string, Slicer};
use pyiter::PyFileIter;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileLike(PathBuf);

impl FromPyObject<'_> for FileLike {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let file_name: String = ob
            .extract::<String>()
            .or_else(|_| ob.getattr("name")?.extract::<String>())
            .or_else(|_| {
                Err(PyErr::new::<PyTypeError, _>(
                    "Not a path or file-like object",
                ))
            })?;
        Ok(FileLike(file_name.into()))
    }
}

impl AsRef<Path> for FileLike {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[pymodule]
fn preprocess_cancellation(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(preprocess_slicer, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_cura, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_ideamaker, m)?)?;
    m.add_function(wrap_pyfunction!(preprocess_m486, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn preprocess_slicer(file_like: FileLike) -> PyResult<PyFileIter> {
    let mut src = BufReader::new(File::open(&file_like)?);
    let mut objects = slicers::slic3r::list_objects(&mut src)?;
    Ok(match rewrite(file_like.as_ref(), &mut objects).unwrap() {
        None => PyFileIter(None),
        Some(dst) => PyFileIter(Some(BufReader::new(dst))),
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
pub fn preprocess_ideamaker(file_path: FileLike) -> PyResult<()> {
    todo!()
}

#[pyfunction]
pub fn preprocess_m486(file_path: FileLike) -> PyResult<()> {
    todo!()
}
