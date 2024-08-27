use pyo3::prelude::*;


#[pymodule]
pub fn string_operations(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concatenate, m)?)?;
    m.add_function(wrap_pyfunction!(reverse, m)?)?;
    Ok(())
}

#[allow(dead_code)]
#[pyfunction]
fn concatenate(a: &str, b: &str) -> PyResult<String> {
    Ok(format!("{}{}", a, b))
}

#[allow(dead_code)]
#[pyfunction]
fn reverse(s: &str) -> PyResult<String> {
    Ok(s.chars().rev().collect())
}
