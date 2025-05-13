use std::fmt;

use num_complex::Complex;

use crate::quantum::types::{
    quantum_gate::QuantumGate, quantum_operators::QuantumOperator,
    quantum_position::QuantumPosition,
};

impl QuantumGate {
    /// The [`NOT`](QuantumGate::NOT) gate flips the amplitudes of the $|0\rangle$ and $|1\rangle$ states,
    /// analagous to a typical NOT gate.
    ///
    /// The gate can be represented by the following matrix:
    /// $$\begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}$$
    ///
    /// # Examples
    /// The [`NOT`](QuantumGate::NOT) gate can flip the amplitude of a qubit such that:
    ///
    /// $$ \begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}
    /// \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} =
    /// \begin{pmatrix} \beta \\\ \alpha \end{pmatrix} $$
    /// ```rust
    /// use rquant::quantum::types::quantum_gate::QuantumGate;
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn flip_qubit(qubit: &Qubit) -> Qubit {
    ///     qubit.apply_gate(&QuantumGate::NOT)
    /// }
    /// ```
    ///
    /// The [`NOT`](QuantumGate::NOT) gate can also be expressed with the `!` symbol:
    /// ```rust
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn flip_qubit(qubit: Qubit) -> Qubit {
    ///     !qubit
    /// }
    /// ```
    pub const NOT: QuantumGate = QuantumGate::new(QuantumOperator::NOT);

    /// The [`ROTATE`](QuantumGate::ROTATE) gate rotates a [`Qubit`](crate::quantum::types::qubit::Qubit)
    /// 180 degrees around it's Y-axis.
    ///
    /// The gate can be represented by the following matrix:
    /// $$\begin{pmatrix} 0 & -i \\\ i & 0 \end{pmatrix}$$
    ///
    /// # Example
    /// The [`ROTATE`](QuantumGate::ROTATE) gate can phase the qubit such that:
    ///
    /// $$ \begin{pmatrix} 0 & -i \\\ i & 0 \end{pmatrix}
    /// \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} =
    /// \begin{pmatrix} -i \beta \\\ i \alpha \end{pmatrix} =
    /// -\beta|0\rangle + \alpha|1\rangle $$
    /// ```rust
    /// use rquant::quantum::types::quantum_gate::QuantumGate;
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn rotate_qubit(qubit: &Qubit) -> Qubit {
    ///     qubit.apply_gate(&QuantumGate::ROTATE)
    /// }
    /// ```
    pub const ROTATE: QuantumGate = QuantumGate::new(QuantumOperator::ROTATE);

    /// The [`PHASE`](QuantumGate::PHASE) gate leaves the state of a [`Qubit`](crate::quantum::types::qubit::Qubit)
    /// unchanged, and flips the phase of the $|1\rangle$ state by $\pi$.
    ///
    /// The gate can be represented by the following matrix:
    /// $$\begin{pmatrix} 1 & 0 \\\ 0 & -1 \end{pmatrix}$$
    ///
    /// # Example
    /// The [`PHASE`](QuantumGate::PHASE) gate can phase and flip the qubit such that:
    ///
    /// $$ \begin{pmatrix} 1 & 0 \\\ 0 & -1 \end{pmatrix}
    /// \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} =
    /// \begin{pmatrix} \alpha \\\ -\beta \end{pmatrix} =
    /// \alpha|0\rangle - \beta|1\rangle $$
    /// ```rust
    /// use rquant::quantum::types::quantum_gate::QuantumGate;
    /// use rquant::quantum::types::qubit::Qubit;
    ///
    /// fn phase_qubit(qubit: &Qubit) -> Qubit {
    ///     qubit.apply_gate(&QuantumGate::PHASE)
    /// }
    /// ```
    pub const PHASE: QuantumGate = QuantumGate::new(QuantumOperator::PHASE);

    /// The [`SUPERPOSITION`](QuantumGate::SUPERPOSITION) turns the amplitude of the $|0\rangle$ and $|1\rangle$
    /// states of a [`Qubit`](crate::quantum::types::qubit::Qubit) into an equal superposition of $|0\rangle$
    /// and $|1\rangle$.
    ///
    /// Also referred to as a "Hadamard gate".
    ///
    /// The SUPERPOSITION [`QuantumOperator`] can be represented by the following matrix:
    /// $$S=\frac{1}{\sqrt{2}}\begin{pmatrix} 1 & 1 \\\ 1 & -1 \end{pmatrix}$$
    ///
    /// The [`SUPERPOSITION`](QuantumGate::SUPERPOSITION) gate can modify the amplitude of a
    /// [`Qubit`](crate::quantum::types::qubit::Qubit) such that:
    ///
    /// $$H|0\rangle = \frac{1}{\sqrt{2}}(|0\rangle + |1\rangle)$$
    /// $$H|1\rangle = \frac{1}{\sqrt{2}}(|0\rangle - |1\rangle)$$
    /// ```rust
    /// use rquant::quantum::types::{quantum_gate::QuantumGate, qubit::Qubit};
    ///
    /// fn superposition_qubit(qubit: &Qubit) -> Qubit {
    ///     qubit.apply_gate(&QuantumGate::SUPERPOSITION)
    /// }
    /// ```
    pub const SUPERPOSITION: QuantumGate = QuantumGate::new(QuantumOperator::SUPERPOSITION);

    /// [`QuantumGate::new`] will create a [`QuantumGate`] to transform a [`Qubit`](crate::quantum::types::qubit::Qubit)
    /// in complex vector space based on the provided [`QuantumOperator`].
    ///
    /// # Example
    /// [`QuantumGate::new`] can be used to create a new [`QuantumGate`] based off of the
    /// [`QuantumOperator::ROTATE`] operation:
    /// ```rust
    /// use rquant::quantum::types::{quantum_gate::QuantumGate, quantum_operators::QuantumOperator};
    ///
    /// fn create_rotated_quantum_gate() -> QuantumGate {
    ///     QuantumGate::new(QuantumOperator::ROTATE)
    /// }
    /// ```
    pub const fn new(operator: QuantumOperator) -> Self {
        let transform = match operator {
            QuantumOperator::NOT => [QuantumPosition::ONE, QuantumPosition::ZERO],
            QuantumOperator::ROTATE => [
                QuantumPosition::BACK_QUARTER_TURN,
                QuantumPosition::QUARTER_TURN,
            ],
            QuantumOperator::PHASE => [QuantumPosition::ZERO, QuantumPosition::FLIP],
            QuantumOperator::SUPERPOSITION => {
                let factor = 1.0 / std::f64::consts::SQRT_2;
                [
                    QuantumPosition::new(Complex::new(factor, 0.0), Complex::new(factor, 0.0)),
                    QuantumPosition::new(Complex::new(factor, 0.0), Complex::new(-factor, 0.0)),
                ]
            }
        };
        QuantumGate {
            operator,
            transform,
        }
    }
}

/// Implement the [`fmt::Display`] trait for [`QuantumGate`].
impl fmt::Display for QuantumGate {
    /// Will return a [`String`] representation of a [`QuantumGate`].
    ///
    /// Produces matrix notation of the [`QuantumGate`]. For instance, [`QuantumGate::NOT`]
    /// will be presented as:
    /// $$\begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}$$
    ///
    /// # Example
    /// Can be used to print a [`QuantumGate`] to the console:
    /// ```rust
    /// use rquant::quantum::types::quantum_gate::QuantumGate;
    ///
    /// fn print_quantum_gate(quantum_gate: QuantumGate) {
    ///     println!("{quantum_gate}");
    /// }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n┏          ┓\n┃ {} {} ┃\n┃ {} {} ┃\n┗          ┛",
            self.transform[0].initial_position,
            self.transform[1].initial_position,
            self.transform[0].possible_position,
            self.transform[1].possible_position
        )
    }
}
