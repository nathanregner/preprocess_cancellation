use crate::DEFAULT_BUF_SIZE;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tempfile::NamedTempFile;

#[pyclass]
pub struct FileIter(pub BufReader<File>);

impl From<File> for FileIter {
    fn from(file: File) -> Self {
        Self(BufReader::with_capacity(DEFAULT_BUF_SIZE, file))
    }
}

impl From<BufReader<File>> for FileIter {
    fn from(reader: BufReader<File>) -> Self {
        Self(reader)
    }
}

impl From<NamedTempFile> for FileIter {
    fn from(file: NamedTempFile) -> Self {
        file.into_file().into()
    }
}

#[pymethods]
impl FileIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<String>> {
        let mut line = String::new();
        Ok(match slf.0.read_line(&mut line)? {
            0 => None,
            _ => Some(line),
        })
    }
}
