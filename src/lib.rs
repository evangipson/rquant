//! # rquant
//! [`rquant`](crate) is a quantum computing library written entirely in rust.
//!
//! It allows for qubit measurement and basic quantum logic in complex vector space.

/// [`quantum`] is a collection of [`types`](crate::quantum::types),
/// [`constants`](crate::quantum::constants), and [`behaviors`](crate::quantum::behaviors)
/// for quantum computing.
pub mod quantum {
    /// [`types`] is a collection of `struct` and `enum` that is used to hold data for
    /// [`behaviors`].
    pub mod types {
        /// [`quantum_gate`] contains all the [`types`](crate::quantum::types) for quantum logic
        /// gates.
        pub mod quantum_gate;

        /// [`quantum_operators`] contains all the [`types`](crate::quantum::types) for logical
        /// operation types.
        pub mod quantum_operators;

        /// [`quantum_position`] contains all the [`types`](crate::quantum::types) for expressing
        /// [`Qubit`](crate::quantum::types::qubit::Qubit) position in complex vector space.
        pub mod quantum_position;

        /// [`qubit`] contains all the [`types`](crate::quantum::types) for anything related to
        /// qubits.
        pub mod qubit;
    }

    /// [`constants`] is a collection of `const` values that will never change, and are
    /// used throughout the [`quantum`](crate::quantum) module.
    pub mod constants {
        /// [`ket`] is a collection of `const` [`Qubit`](crate::quantum::types::qubit::Qubit) states
        /// expressed in ["Bra-Ket" form](https://en.wikipedia.org/wiki/Bra-ket_notation).
        pub mod ket;
    }

    /// [`behaviors`] is a collection of implementations for each [`type`](crate::quantum::types).
    pub mod behaviors {
        /// [`quantum_gate`](crate::quantum::types::quantum_gate) contains all the
        /// [`behaviors`](crate::quantum::behaviors) for quantum logic gates.
        pub mod quantum_gate;

        /// [`quantum_position`](crate::quantum::types::quantum_position) contains all the
        /// [`behaviors`](crate::quantum::behaviors) for expressing qubit position in complex
        /// vector space.
        pub mod quantum_position;

        /// [`qubit`](crate::quantum::types::qubit) contains all the
        /// [`behaviors`](crate::quantum::behaviors) for anything related to qubits.
        pub mod qubit;
    }
}

/// [`logger`] is a collection of [`types`](crate::logger::types), `behaviors`, and [`macros`](crate::logger::macros)
/// for logging to the console.
pub mod logger {
    /// [`types`] is a collection of `struct` and `enum` that is used to hold data for
    /// [`logger`](crate::logger) `behaviors`.
    pub mod types {
        /// [`logger`] contains all the [`types`](crate::logger::types) for logging.
        pub mod logger;
    }

    /// [`macros`] is a collection of macros that are used for the [`logger`](crate::logger).
    pub mod macros {
        /// [`log`] contains all the `macro_rules` for the [`logger`](crate::logger).
        pub mod log;
    }
}
