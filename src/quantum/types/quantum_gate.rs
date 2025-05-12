use crate::quantum::types::{
    quantum_operators::QuantumOperator, quantum_position::QuantumPosition,
};

/// [`QuantumGate`] is a logic gate used to measure one or many
/// [`Qubits`](crate::quantum::types::qubit::Qubit).
pub struct QuantumGate {
    /// An identifier for a [`QuantumGate`] that determines what the `transform` is.
    pub operator: QuantumOperator,

    /// Two [`QuantumPositions`](QuantumPosition) that make up a quantum velocity to
    /// move a [`Qubit`](crate::quantum::types::qubit::Qubit).
    pub transform: [QuantumPosition; 2],
}
