use pyo3::prelude::*;
use std::io::{BufRead, BufReader};
use tempfile::NamedTempFile;

#[pyclass]
pub struct FileIter(pub Option<BufReader<NamedTempFile>>);

#[pymethods]
impl FileIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<String>> {
        if let Some(reader) = &mut slf.0 {
            let mut line = String::new();
            match reader.read_line(&mut line)? {
                0 => {
                    slf.0.take();
                    Ok(None)
                }
                _ => Ok(Some(line)),
            }
        } else {
            Ok(None)
        }
    }
}
