use num_complex::Complex;

/// A [`QuantumPosition`] consists of two [`Complex<f64>`] positions: an `initial_position`,
/// and a `possible_position`. The `initial_position` is the starting point in complex vector
/// space, and the `possible_position` is the superposition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantumPosition {
    /// A [`Complex<f64>`] number that represents an initial position in complex vector space.
    pub initial_position: Complex<f64>,

    /// A [`Complex<f64>`] number that represents a superposition of `initial_position`.
    pub possible_position: Complex<f64>,
}
