mod composition;
mod primitive;
mod sdf;
mod vec3;

use pyo3::prelude::*;

#[pymodule]
fn sdflit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<sdf::DynSDF>()?;
    m.add_function(wrap_pyfunction!(composition::merge, m)?)?;
    m.add_function(wrap_pyfunction!(composition::intersect, m)?)?;
    m.add_function(wrap_pyfunction!(composition::subtract, m)?)?;

    // primitive
    m.add_class::<primitive::FrustumCone>()?;
    m.add_class::<primitive::RoundCone>()?;
    m.add_class::<primitive::Sphere>()?;

    Ok(())
}
