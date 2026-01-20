// SPDX-FileCopyrightText: 2023-2026 Zexin Yuan <aim@yzx9.xyz>
//
// SPDX-License-Identifier: Apache-2.0

//! SDF (Signed Distance Field) library with Python bindings
//!
//! This library provides primitives, composition operations, and samplers
//! for working with signed distance fields.

#![warn(
    trivial_casts,
    trivial_numeric_casts,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    clippy::dbg_macro,
    clippy::indexing_slicing,
    clippy::pedantic
)]
// Allow certain clippy lints that are too restrictive for this crate
#![allow(clippy::many_single_char_names)]

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
fn sdflit(m: &Bound<'_, PyModule>) -> PyResult<()> {
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
