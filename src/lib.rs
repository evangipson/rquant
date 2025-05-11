//! # rquant
//! [`rquant`](crate) is a quantum computing library written entirely in rust.
//!
//! It allows for qubit measurement and basic quantum logic in complex vector space.

/// [`types`] is a collection of `struct` and `enum` that is used to hold data for
/// [`behaviors`].
pub mod types {
    /// [`quantum_gate`] contains all the [`types`](crate::types) for quantum logic
    /// gates.
    pub mod quantum_gate;

    /// [`quantum_operators`] contains all the [`types`](crate::types) for logical
    /// operation types.
    pub mod quantum_operators;

    /// [`quantum_position`] contains all the [`types`](crate::types) for expressing
    /// [`Qubit`](crate::types::qubit::Qubit) position in complex vector space.
    pub mod quantum_position;

    /// [`qubit`] contains all the [`types`](crate::types) for anything related to
    /// qubits.
    pub mod qubit;
}

/// [`constants`] is a collection of `const` values that will never change, and are
/// used throughout [`rquant`](crate).
pub mod constants {
    /// [`ket`] is a collection of `const` [`Qubit`](crate::types::qubit::Qubit) states
    /// expressed in ["Bra-Ket" form](https://en.wikipedia.org/wiki/Bra-ket_notation).
    pub mod ket;
}

/// [`behaviors`] is a collection of implementations for each [`type`](crate::types).
pub mod behaviors {
    /// [`quantum_gate`] contains all the [`behaviors`](crate::behaviors) for quantum
    /// logic gates.
    pub mod quantum_gate;

    /// [`quantum_position`] contains all the [`behaviors`](crate::behaviors) for
    /// expressing qubit
    /// position in complex vector space.
    pub mod quantum_position;

    /// [`qubit`] contains all the [`behaviors`](crate::behaviors) for anything related
    /// to qubits.
    pub mod qubit;
}
