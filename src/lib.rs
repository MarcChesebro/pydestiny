use pyo3::prelude::*;

/// wrapper
#[pymodule]
fn pydestiny(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "hello_world")]
    fn hello_world(_py: Python) -> PyResult<String> {
        let out = "Hello_world";
        Ok(out.to_string())
    }

    Ok(())
}
