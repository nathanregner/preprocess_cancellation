use pyo3::exceptions::PyTypeError;
use pyo3::prelude::PyAnyMethods;
use pyo3::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileLike(PathBuf);

impl FromPyObject<'_> for FileLike {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let file_name: String = ob
            .extract::<String>()
            .or_else(|_| ob.getattr("name")?.extract::<String>())
            .map_err(|_| PyErr::new::<PyTypeError, _>("Not a path or file-like object"))?;
        Ok(FileLike(file_name.into()))
    }
}

impl AsRef<Path> for FileLike {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
