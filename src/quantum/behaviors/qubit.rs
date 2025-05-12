use std::{fmt, ops};

use num_complex::Complex;
use rand::Rng;

use crate::quantum::types::{
    quantum_gate::QuantumGate, quantum_operators::QuantumOperator,
    quantum_position::QuantumPosition, qubit::Qubit,
};

impl Qubit {
    /// [`Qubit::new`] will create a new [`Qubit`] with a [`QuantumPosition`]
    /// in complex vector space.
    pub fn new(position: QuantumPosition) -> Self {
        // The amplitudes of the qubit's initial and possible magnitude must equal 1,
        // or the qubit has an invalid position.
        assert!(position.has_valid_amplitude(), "Invalid qubit positions");

        Qubit {
            positions: vec![position],
        }
    }

    /// [`Qubit::zero`] will return a new [`Qubit`] with it's position set to [`QuantumPosition::ZERO`].
    ///
    /// [`Qubit::zero`] can be represented by the following matrix:
    /// $$ 0 = \begin{pmatrix}1 \\\ 0 \end{pmatrix} $$
    pub fn zero() -> Self {
        Qubit::new(QuantumPosition::ZERO)
    }

    /// [`Qubit::one`] will return a new [`Qubit`] with it's position set to [`QuantumPosition::ONE`].
    ///
    /// [`Qubit::one`] can be represented by the following matrix:
    /// $$ 1 = \begin{pmatrix} 0 \\\ 1 \end{pmatrix} $$
    pub fn one() -> Self {
        Qubit::new(QuantumPosition::ONE)
    }

    /// [`Qubit::flip`] will return a new [`Qubit`] with it's position set to [`QuantumPosition::FLIP`].
    ///
    /// [`Qubit::flip`] can be represented by the following matrix:
    /// $$ F = \begin{pmatrix} 0 \\\ -1 \end{pmatrix} $$
    pub fn flip() -> Self {
        Qubit::new(QuantumPosition::FLIP)
    }

    /// [`Qubit::half_turn`] will return a new [`Qubit`] with it's position set to [`QuantumPosition::HALF_TURN`].
    ///
    /// [`Qubit::half_turn`] can be represented by the following matrix:
    /// $$ Ht = \begin{pmatrix} i \\\ 0 \end{pmatrix} $$
    pub fn half_turn() -> Self {
        Qubit::new(QuantumPosition::HALF_TURN)
    }

    /// [`Qubit::back_half_turn`] will return a new [`Qubit`] with it's position set to [`QuantumPosition::BACK_HALF_TURN`].
    ///
    /// [`Qubit::back_half_turn`] can be represented by the following matrix:
    /// $$ B(Ht) = \begin{pmatrix} 0 \\\ -i \end{pmatrix} $$
    pub fn back_half_turn() -> Self {
        Qubit::new(QuantumPosition::BACK_HALF_TURN)
    }

    /// [`Qubit::move`] will move the [`Qubit`] that calls it to a new
    /// [`QuantumPosition`] in complex vector space.
    pub fn r#move(mut self, new_position: QuantumPosition) {
        self.positions.push(new_position);
    }

    /// [`Qubit::apply_gate`] will apply a [`QuantumGate`] to the [`QuantumPosition`]
    /// of the [`Qubit`] that calls it.
    pub fn apply_gate(&self, gate: &QuantumGate) -> Self {
        let first_gate = gate.transform[0];
        let second_gate = gate.transform[1];

        let qubit_position = QuantumPosition::new(
            first_gate.initial_position * self.initial_position()
                + first_gate.possible_position * self.possible_position(),
            second_gate.initial_position * self.initial_position()
                + second_gate.possible_position * self.possible_position(),
        );

        Qubit::new(qubit_position)
    }

    /// [`Qubit::measure`] will measure a [`Qubit`] position in complex vector space,
    /// determined by [`Qubit::initial_position`], and return a [`bool`] for it's
    /// "truthy" state.
    pub fn measure(&self) -> bool {
        let prob_zero = self.initial_position().norm_sqr();
        let mut rng = rand::rng();
        rng.random_bool(prob_zero)
    }

    /// [`Qubit::initial_position`] will retrieve the current initial position
    /// of the [`Qubit`] that calls it.
    pub fn initial_position(&self) -> Complex<f64> {
        self.position().initial_position
    }

    /// [`Qubit::possible_position`] will retrieve the current possible position
    /// of the [`Qubit`] that calls it.
    pub fn possible_position(&self) -> Complex<f64> {
        self.position().possible_position
    }

    /// [`Qubit::position`] will retrieve the current position of the
    /// [`Qubit`] that calls it.
    fn position(&self) -> QuantumPosition {
        self.positions
            .first()
            .cloned()
            .expect("Must have an initial qubit position.")
    }
}

/// Implement the `!` operator for [`Qubit`].
impl ops::Not for Qubit {
    type Output = Self;

    /// Flips amplitudes of a [`Qubit`], analagous to a typical NOT gate.
    ///
    /// The gate can be represented by the following matrix:
    /// $$\begin{pmatrix}0&1\\\1&0\end{pmatrix}$$
    ///
    /// # Example
    /// The [`NOT`](QuantumGate::NOT) [`QuantumGate`] can be expressed on a
    /// [`Qubit`] using the `!` symbol:
    /// ```rust
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn flip_qubit(qubit: Qubit) -> Qubit {
    ///     !qubit
    /// }
    /// ```
    fn not(self) -> Self::Output {
        self.apply_gate(&QuantumGate::new(QuantumOperator::NOT))
    }
}

/// Implement the [`fmt::Display`] trait for [`Qubit`].
impl fmt::Display for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let initial_includes_imaginary = self.initial_position().im != 0.0;
        let alpha = if initial_includes_imaginary {
            self.initial_position().to_string()
        } else {
            self.initial_position().re.to_string()
        };

        let possible_includes_imaginary = self.possible_position().im != 0.0;
        let beta = if possible_includes_imaginary {
            self.possible_position().to_string()
        } else {
            self.possible_position().re.to_string()
        };

        write!(f, "{}|{}〉", alpha, beta)
    }
}
