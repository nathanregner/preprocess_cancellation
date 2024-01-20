mod bounding_box;
mod slicers;

use pyo3::prelude::*;

// preprocess_slicer,
// preprocess_cura,
// preprocess_ideamaker,
// preprocess_m486

#[pymodule]
fn preprocess_cancellation(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(preprocess_slicer, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn preprocess_slicer(file_path: &str) -> PyResult<()> {
    println!("preprocess_slicer: {}", file_path);
    Ok(())
}
