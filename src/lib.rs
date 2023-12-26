mod accelerator;
mod composition;
mod material;
mod object;
mod primitive;
mod sampler;
mod scene;
mod sdf;
mod solid_geometry;
mod vec3;

use pyo3::prelude::*;

#[pymodule]
fn sdflit(_py: Python, m: &PyModule) -> PyResult<()> {
    // SDF and Primitives
    m.add_class::<sdf::DynSDF>()?;
    m.add_class::<primitive::FrustumCone>()?;
    m.add_class::<primitive::RoundCone>()?;
    m.add_class::<primitive::Sphere>()?;

    // Composition
    m.add_function(wrap_pyfunction!(composition::merge, m)?)?;
    m.add_function(wrap_pyfunction!(composition::intersect, m)?)?;
    m.add_function(wrap_pyfunction!(composition::subtract, m)?)?;

    // Material
    m.add_class::<material::DynMaterial>()?;
    m.add_class::<material::ColoredMaterial>()?;
    m.add_class::<material::LinearGradientMaterial>()?;

    // Object
    m.add_class::<object::DynObject>()?;
    m.add_class::<object::SDFObject>()?;

    // Scene
    m.add_class::<scene::DynScene>()?;
    m.add_class::<scene::ObjectsScene>()?;

    // Sampler
    m.add_class::<sampler::UniformSampler>()?;
    m.add_class::<sampler::RangeSampler>()?;

    Ok(())
}
