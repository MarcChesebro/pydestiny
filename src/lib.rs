//! This is a wrapper for the destiny crate.
//! 
//! Destiny is a crate for dice rolling utilities

use pyo3::prelude::*;
use destiny;

/// wrapper for the destiny rust crate
#[pymodule]
fn pydestiny(_py: Python, m: &PyModule) -> PyResult<()> {

    /// wrapper for destiny::parse_dice_string
    #[pyfn(m, "parse_dice_string")]
    fn parse_dice_string(_py: Python, string: &str) -> PyResult<i64> {
        Ok(destiny::parse_dice_string(string))
    }

    Ok(())
}
