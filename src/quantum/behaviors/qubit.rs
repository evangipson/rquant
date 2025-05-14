use crate::quantum::types::{
    quantum_gate::QuantumGate, quantum_operators::QuantumOperator,
    quantum_position::QuantumPosition, qubit::Qubit,
};
use num_complex::Complex;
use rand::Rng;
use std::{fmt, ops};

impl Qubit {
    /// [`Qubit::new`] will create a new [`Qubit`] with a [`QuantumPosition`]
    /// in complex vector space.
    ///
    /// # Example
    /// [`Qubit::new`] can be used to create a new [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn create_qubit() -> Qubit {
    ///     Qubit::new(QuantumPosition::ZERO)
    /// }
    /// ```
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
    ///
    /// # Example
    /// [`Qubit::zero`] can be used to get a zero [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn get_zero_qubit() -> Qubit {
    ///     Qubit::zero()
    /// }
    /// ```
    pub fn zero() -> Self {
        Qubit::new(QuantumPosition::ZERO)
    }

    /// [`Qubit::one`] (also referred to as an "identity qubit") will return a new [`Qubit`] with it's
    /// position set to [`QuantumPosition::ONE`].
    ///
    /// [`Qubit::one`] can be represented by the following matrix:
    /// $$ 1 = \begin{pmatrix} 0 \\\ 1 \end{pmatrix} $$
    ///
    /// # Example
    /// [`Qubit::one`] can be used to get an identity [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn get_identity_qubit() -> Qubit {
    ///     Qubit::one()
    /// }
    /// ```
    pub fn one() -> Self {
        Qubit::new(QuantumPosition::ONE)
    }

    /// [`Qubit::flip`] will return a new [`Qubit`] with it's position set to [`QuantumPosition::FLIP`].
    ///
    /// [`Qubit::flip`] can be represented by the following matrix:
    /// $$ F = \begin{pmatrix} 0 \\\ -1 \end{pmatrix} $$
    ///
    /// # Example
    /// [`Qubit::flip`] can be used to get a flipped [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn get_flipped_qubit() -> Qubit {
    ///     Qubit::flip()
    /// }
    /// ```
    pub fn flip() -> Self {
        Qubit::new(QuantumPosition::FLIP)
    }

    /// [`Qubit::quarter_turn`] will return a new [`Qubit`] with it's position set to
    /// [`QuantumPosition::QUARTER_TURN`].
    ///
    /// [`Qubit::quarter_turn`] can be represented by the following matrix:
    /// $$ Qt = \begin{pmatrix} i \\\ 0 \end{pmatrix} $$
    ///
    /// # Example
    /// [`Qubit::quarter_turn`] can be used to get a quarter-turned [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn get_quarter_turned_qubit() -> Qubit {
    ///     Qubit::quarter_turn()
    /// }
    /// ```
    pub fn quarter_turn() -> Self {
        Qubit::new(QuantumPosition::QUARTER_TURN)
    }

    /// [`Qubit::back_quarter_turn`] will return a new [`Qubit`] with it's position set to
    /// [`QuantumPosition::BACK_QUARTER_TURN`].
    ///
    /// [`Qubit::back_quarter_turn`] can be represented by the following matrix:
    /// $$ B(Qt) = \begin{pmatrix} 0 \\\ -i \end{pmatrix} $$
    ///
    /// # Example
    /// [`Qubit::back_quarter_turn`] can be used to get a backwards quarter-turned [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn get_backwards_quarter_turned_qubit() -> Qubit {
    ///     Qubit::back_quarter_turn()
    /// }
    /// ```
    pub fn back_quarter_turn() -> Self {
        Qubit::new(QuantumPosition::BACK_QUARTER_TURN)
    }

    /// [`Qubit::update`] will move the [`Qubit`] that calls it to a new [`QuantumPosition`] in
    /// complex vector space, and maintains the old position inside of [`Qubit::positions`].
    ///
    /// # Example
    /// [`Qubit::update`] can be used to update the position of a [`Qubit`].
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_position::QuantumPosition};
    ///
    /// fn move_qubit(mut qubit: Qubit, new_position: QuantumPosition) {
    ///     qubit.update(new_position);
    /// }
    /// ```
    pub fn update(&mut self, new_position: QuantumPosition) {
        self.positions.push(new_position);
    }

    /// [`Qubit::apply_gate`] will apply a [`QuantumGate`] to the [`QuantumPosition`]
    /// of the [`Qubit`] that calls it, then return the modified [`Qubit`].
    ///
    /// It is how a [`Qubit`] has logic applied to it.
    ///
    /// # Example
    /// [`Qubit::apply_gate`] can be used to apply a NOT gate to a [`Qubit`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_gate::QuantumGate};
    ///
    /// fn invert_qubit_amplitudes(qubit: Qubit) -> Qubit {
    ///     qubit.apply_gate(&QuantumGate::NOT)
    /// }
    /// ```
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
    ///
    /// It is how [`Qubit`] superposition is observed.
    ///
    /// # Example
    /// [`Qubit::measure`] can be used to observe a [`Qubit`] after it passes through
    /// a [`QuantumGate`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit::Qubit, quantum_gate::QuantumGate};
    ///
    /// fn observe_phased_qubit(qubit: Qubit) -> bool {
    ///     qubit.apply_gate(&QuantumGate::PHASE).measure()
    /// }
    /// ```
    pub fn measure(&self) -> bool {
        let prob_zero = self.initial_position().norm_sqr();
        let mut rng = rand::rng();
        rng.random_bool(prob_zero)
    }

    /// [`Qubit::initial_position`] will retrieve the current initial position
    /// of the [`Qubit`] that calls it.
    ///
    /// # Example
    /// [`Qubit::initial_position`] can be used to get the initial position of
    /// a [`Qubit`]:
    /// ```rust
    /// use num_complex::Complex;
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn get_qubit_initial_position(qubit: Qubit) -> Complex<f64> {
    ///     qubit.initial_position()
    /// }
    /// ```
    pub fn initial_position(&self) -> Complex<f64> {
        self.position().initial_position
    }

    /// [`Qubit::possible_position`] will retrieve the current possible position
    /// of the [`Qubit`] that calls it.
    ///
    /// # Example
    /// [`Qubit::possible_position`] can be used to get the possible position of
    /// a [`Qubit`]:
    /// ```rust
    /// use num_complex::Complex;
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn get_qubit_possible_position(qubit: Qubit) -> Complex<f64> {
    ///     qubit.possible_position()
    /// }
    /// ```
    pub fn possible_position(&self) -> Complex<f64> {
        self.position().possible_position
    }

    /// [`Qubit::position`] will retrieve the current position of the [`Qubit`]
    /// that calls it.
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

    /// Flips amplitudes of a [`Qubit`] (analagous to a typical NOT gate), and can be expressed
    /// by prefixing the `!` symbol to a [`Qubit`].
    ///
    /// The gate can be represented by the following matrix:
    /// $$\begin{pmatrix}0&1\\\1&0\end{pmatrix}$$
    ///
    /// # Example
    /// Can be used to invert a [`Qubit`]:
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
    /// Gets the [`String`] representation of a [`Qubit`], based on it's [`QuantumPosition`].
    ///
    /// It will produce the ["Bra-Ket"](https://en.wikipedia.org/wiki/Bra-ket_notation) notation
    /// of the [`Qubit`]. For instance, [`Qubit::zero`] will be presented as:
    /// $$ 1|0\rangle $$
    ///
    /// While a more complex [`Qubit`], such as [`Qubit::back_quarter_turn`] will be presented as:
    /// $$ 0|0-1i\rangle $$
    ///
    /// # Example
    /// Can be used to print a [`Qubit`] to the console:
    /// ```rust
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn print_qubit(qubit: Qubit) {
    ///     println!("{qubit}");
    /// }
    /// ```
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
