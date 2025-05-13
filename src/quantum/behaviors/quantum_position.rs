use num_complex::Complex;

use crate::quantum::{
    constants::ket::{KET_BACK_ROTATION, KET_FLIP, KET_ONE, KET_ROTATION, KET_ZERO},
    types::quantum_position::QuantumPosition,
};

impl QuantumPosition {
    /// The [`ZERO`](QuantumPosition::ZERO) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ 0 = \begin{pmatrix} 1 \\\ 0 \end{pmatrix} $$
    pub const ZERO: QuantumPosition = QuantumPosition::new(KET_ONE, KET_ZERO);

    /// The [`ONE`](QuantumPosition::ONE) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ 1 = \begin{pmatrix} 0 \\\ 1 \end{pmatrix} $$
    pub const ONE: QuantumPosition = QuantumPosition::new(KET_ZERO, KET_ONE);

    /// The [`FLIP`](QuantumPosition::FLIP) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ F = \begin{pmatrix} 0 \\\ -1 \end{pmatrix} $$
    pub const FLIP: QuantumPosition = QuantumPosition::new(KET_ZERO, KET_FLIP);

    /// The [`QUARTER_TURN`](QuantumPosition::QUARTER_TURN) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ Qt = \begin{pmatrix} i \\\ 0 \end{pmatrix} $$
    pub const QUARTER_TURN: QuantumPosition = QuantumPosition::new(KET_ROTATION, KET_ZERO);

    /// The [`BACK_QUARTER_TURN`](QuantumPosition::BACK_QUARTER_TURN) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ B(Qt) = \begin{pmatrix} 0 \\\ -i \end{pmatrix} $$
    pub const BACK_QUARTER_TURN: QuantumPosition =
        QuantumPosition::new(KET_ZERO, KET_BACK_ROTATION);

    /// [`new`](QuantumPosition::new) will create a new [`QuantumPosition`] in complex vector space, given two [`Complex<f64>`]
    /// numbers.
    ///
    /// # Example
    /// [`new`](QuantumPosition::new) can be used to create a new [`QuantumPosition`]:
    /// ```rust
    /// use num_complex::Complex;
    /// use rquant::quantum::types::quantum_position::QuantumPosition;
    ///
    /// fn create_quantum_position() -> QuantumPosition {
    ///     let initial_amplitude = Complex::new(1.0, 0.0);
    ///     let possible_amplitude = Complex::new(1.0, 1.0);
    ///     QuantumPosition::new(initial_amplitude, possible_amplitude)
    /// }
    /// ```
    pub const fn new(initial_position: Complex<f64>, possible_position: Complex<f64>) -> Self {
        QuantumPosition {
            initial_position,
            possible_position,
        }
    }

    /// [`QuantumPosition::has_valid_amplitude`] will ensure that amplitudes meet the crucial rule of superposition:
    /// the sum of the squares of the initial and possible amplitudes must equal one.
    ///
    /// The algorithm can be illustrated in the following statement:
    /// $$ |\alpha|^2 + |\beta|^2 = 1 $$
    ///
    /// # Example
    /// [`QuantumPosition::has_valid_amplitude`] can be used to ensure a [`QuantumPosition`] has valid amplitudes:
    /// ```rust
    /// use rquant::quantum::types::quantum_position::QuantumPosition;
    ///
    /// fn validate_quantum_position(quantum_position: QuantumPosition) -> bool {
    ///     quantum_position.has_valid_amplitude()
    /// }
    /// ```
    pub fn has_valid_amplitude(&self) -> bool {
        let sum_of_squares = self.initial_position.norm_sqr() + self.possible_position.norm_sqr();
        // Allow a small margin of error for floating-point inaccuracy
        (sum_of_squares - 1.0).abs() < 10.0 * f64::EPSILON
    }
}
