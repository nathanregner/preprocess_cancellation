use pyo3::prelude::*;

#[pyclass]
pub struct PyIter {
    iter: Box<dyn Iterator<Item = String> + Send>,
}

#[pymethods]
impl PyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        slf.iter.next()
    }
}

impl<I> From<I> for PyIter
where
    I: Iterator<Item = String> + Send + 'static,
{
    fn from(iter: I) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}
