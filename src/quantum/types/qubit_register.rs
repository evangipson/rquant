use crate::quantum::types::qubit::Qubit;

/// [`QubitRegister`] holds multiple [`Qubits`](crate::quantum::types::qubit::Qubit).
#[derive(Debug, Clone, PartialEq)]
pub struct QubitRegister {
    /// A collection of [`Qubits`](crate::quantum::types::qubit::Qubit).
    pub qubits: Vec<Qubit>,
}
