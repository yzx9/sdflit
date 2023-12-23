mod primitive;
mod sdf;
mod vec3;

use pyo3::prelude::*;

#[pymodule]
fn sdflit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<primitive::Sphere>()?;
    Ok(())
}
