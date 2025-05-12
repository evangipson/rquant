use num_complex::Complex;

/// A state of a [`Qubit`](crate::quantum::types::qubit::Qubit) that is written formally as:
/// $$|0\rangle$$
pub const KET_ZERO: Complex<f64> = Complex::new(0.0, 0.0);

/// A state of a [`Qubit`](crate::quantum::types::qubit::Qubit) that is written formally as:
/// $$|1\rangle$$
pub const KET_ONE: Complex<f64> = Complex::new(1.0, 0.0);

/// A state of a [`Qubit`](crate::quantum::types::qubit::Qubit) that is written formally as:
/// $$|-1\rangle$$
pub const KET_FLIP: Complex<f64> = Complex::new(-1.0, 0.0);

/// A state of a [`Qubit`](crate::quantum::types::qubit::Qubit) that is written formally as:
/// $$|i\rangle$$
pub const KET_ROTATION: Complex<f64> = Complex::new(0.0, 1.0);

/// A state of a [`Qubit`](crate::quantum::types::qubit::Qubit) that is written formally as:
/// $$|-i\rangle$$
pub const KET_BACK_ROTATION: Complex<f64> = Complex::new(0.0, -1.0);
