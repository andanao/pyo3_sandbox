use pyo3::prelude::*;

#[pymodule]
pub fn math_operations(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}

#[allow(dead_code)]
#[pyfunction]
pub fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[allow(dead_code)]
#[pyfunction]
fn multiply(a: i32, b: i32) -> PyResult<i32> {
    Ok(a * b)
}
