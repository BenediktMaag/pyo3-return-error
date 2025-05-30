use chrono::{DateTime, Local, TimeZone};
use pyo3::prelude::*;

#[pyfunction]
fn get_timestamp() -> Option<DateTime<Local>> {
    Local.with_ymd_and_hms(2025, 05, 30, 12, 30, 34).single()
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_return_error(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_timestamp, m)?)?;
    Ok(())
}
