use crate::types::quantum_position::QuantumPosition;

/// [`Qubit`] is an informational bit that can be both 0 and 1
/// simultaneously due to superposition.
#[derive(Debug, Clone, PartialEq)]
pub struct Qubit {
    /// A collection of [`QuantumPosition`] that represents a timeline
    /// of movement in complex vector space.
    ///
    /// The first element represents the current [`Qubit`] position,
    /// and the last element represents the initial [`Qubit`] position.
    pub positions: Vec<QuantumPosition>,
}
