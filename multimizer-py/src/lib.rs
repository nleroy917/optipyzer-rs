use pyo3::prelude::*;

use multimizer::optimizations::optimize_seq_test;

#[pyfunction(name="optimize")]
fn optimize_py(query: &str) -> PyResult<String> {
    Ok(optimize_seq_test(query))
}

#[pymodule]
#[pyo3(name = "multimizer")]
fn multimizer_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(optimize_py, m)?)?;
    Ok(())
}
