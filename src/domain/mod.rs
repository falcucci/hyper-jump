//! Domain layer primitives for hyper-jump.
//!
//! These modules collect the core data types that describe packages and
//! versions. Higher layers (CLI, adapters) should depend on these instead of
//! redefining structures.

pub mod package;
pub mod version;
