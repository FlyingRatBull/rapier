//! Structures and traits related to gravity.
//!
//! # Examples
//!
//! ## No gravity
//!
//! If you don't want any gravity, you can use `()` in PhysicsPipeline::step().
//!
//! ## Uniform
//!
//! If you want gravity to be the same everywhere, you can use `Uniform` or just a `Vector<Real>` in PhysicsPipeline::step().
//!
//! ### Default gravity on earth
//!
//! If you just want to use earths default gravity (9.81m/s²), you can use `Uniform::default()`.

use std::ops::Deref;

use crate::math::{Isometry, Real, Vector};

/// Trait for calculating gravity at a given point
pub trait Gravity {
    /// Get the force that gravity applies at the given position to the given mass.
    ///
    /// The calculated force should be multiplied by scale before returning.
    fn force_at(&self, position: &Isometry<Real>, mass: Real, scale: Real) -> Vector<Real>;
}

impl<G: Gravity> Gravity for Box<G> {
    fn force_at(&self, position: &Isometry<Real>, mass: Real, scale: Real) -> Vector<Real> {
        self.deref().force_at(position, mass, scale)
    }
}

impl Gravity for Box<dyn Gravity> {
    fn force_at(&self, position: &Isometry<Real>, mass: Real, scale: Real) -> Vector<Real> {
        self.deref().force_at(position, mass, scale)
    }
}

impl Gravity for () {
    fn force_at(&self, _position: &Isometry<Real>, _mass: Real, _scale: Real) -> Vector<Real> {
        Vector::default()
    }
}

/// Uniform gravity which produces the same force at every position
pub struct Uniform {
    direction: Vector<Real>,
}

impl Uniform {
    /// Create a new gravity with the specified direction (pointing "down").
    pub fn new(direction: Vector<Real>) -> Self {
        Self {
            direction
        }
    }
}

impl Default for Uniform {
    /// Earths default gravity (9.81m/s²)
    fn default() -> Self {
        Self {
            direction: Vector::y() * -9.81,
        }
    }
}

impl Gravity for Uniform {
    fn force_at(&self, _position: &Isometry<Real>, mass: Real, scale: Real) -> Vector<Real> {
        self.direction.clone() * mass * scale
    }
}

impl Gravity for Vector<Real> {
    fn force_at(&self, _position: &Isometry<Real>, mass: Real, scale: Real) -> Vector<Real> {
        self.clone() * mass * scale
    }
}

impl<'a> Gravity for &'a Vector<Real> {
    fn force_at(&self, _position: &Isometry<Real>, mass: Real, scale: Real) -> Vector<Real> {
        *self.clone() * mass * scale
    }
}