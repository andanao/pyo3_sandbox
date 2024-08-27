use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod math_operations;
pub mod string_operations;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pymodule]
fn pyo3_test(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(double, m)?);

    let _ = m.add_wrapped(wrap_pymodule!(math_operations::math_operations));
    let _ = m.add_wrapped(wrap_pymodule!(string_operations::string_operations));

    Ok(())
}
