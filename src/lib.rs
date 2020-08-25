use pyo3::prelude::*;
use destiny;

/// wrapper
#[pymodule]
fn pydestiny(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "hello_world")]
    fn hello_world(_py: Python) -> PyResult<String> {
        let out = "Hello_world";
        Ok(out.to_string())
    }

    #[pyfn(m, "parse_dice_string")]
    fn parse_dice_string(_py: Python, string: &str) -> PyResult<i64> {
        Ok(destiny::parse_dice_string(string))
    }

    Ok(())
}
