use num_complex::Complex;

use crate::{
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

    /// The [`HALF_TURN`](QuantumPosition::HALF_TURN) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ Ht = \begin{pmatrix} 0 \\\ i \end{pmatrix} $$
    pub const HALF_TURN: QuantumPosition = QuantumPosition::new(KET_ZERO, KET_ROTATION);

    /// The [`BACK_HALF_TURN`](QuantumPosition::BACK_HALF_TURN) [`QuantumPosition`] can be represented by the following matrix:
    /// $$ B(Ht) = \begin{pmatrix} 0 \\\ -i \end{pmatrix} $$
    pub const BACK_HALF_TURN: QuantumPosition = QuantumPosition::new(KET_ZERO, KET_BACK_ROTATION);

    /// [`new`](QuantumPosition::new) will create a new [`QuantumPosition`] in complex vector space, given two [`Complex<f64>`]
    /// numbers.
    pub const fn new(initial_position: Complex<f64>, possible_position: Complex<f64>) -> Self {
        QuantumPosition {
            initial_position,
            possible_position,
        }
    }
}
