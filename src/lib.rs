//! # rquant
//! [`rquant`](crate) is a quantum computing library for qubit manipulation and observation.
//!
//! It allows for qubit measurement and basic quantum logic in complex vector space.

/// [`quantum`] is a collection of [`types`](crate::quantum::types), [`constants`](crate::quantum::constants),
/// and [`behaviors`](crate::quantum::behaviors) for quantum measurements and observations.
pub mod quantum {
    /// [`types`] is a collection of `struct` and `enum` that is used to hold data for
    /// [`behaviors`].
    pub mod types {
        /// [`quantum_gate`](crate::quantum::types::quantum_gate::QuantumGate) contains all the
        /// [`types`](crate::quantum::types) for quantum logic gates.
        pub mod quantum_gate;

        /// [`quantum_operators`](crate::quantum::types::quantum_operators::QuantumOperator) contains
        /// all the [`types`](crate::quantum::types) for logical operation types.
        pub mod quantum_operators;

        /// [`quantum_position`](crate::quantum::types::quantum_position::QuantumPosition) contains all
        /// the [`types`](crate::quantum::types) for expressing [`Qubit`](crate::quantum::types::qubit::Qubit)
        /// position in complex vector space.
        pub mod quantum_position;

        /// [`qubit`](crate::quantum::types::qubit::Qubit) contains all the [`types`](crate::quantum::types)
        /// for anything related to qubits.
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
        /// [`quantum_gate`](crate::quantum::types::quantum_gate::QuantumGate) contains all the
        /// [`behaviors`](crate::quantum::behaviors) for quantum logic gates.
        pub mod quantum_gate;

        /// [`quantum_position`](crate::quantum::types::quantum_position::QuantumPosition) contains all the
        /// [`behaviors`](crate::quantum::behaviors) for expressing qubit position in complex
        /// vector space.
        pub mod quantum_position;

        /// [`qubit`](crate::quantum::types::qubit::Qubit) contains all the
        /// [`behaviors`](crate::quantum::behaviors) for anything related to qubits.
        pub mod qubit;
    }
}

/// [`logger`] is a collection of [`types`](crate::logger::types), [`behaviors`](crate::logger::behaviors),
/// and [`macros`](crate::logger::macros) for logging to the console.
pub mod logger {
    /// [`types`] is a collection of `struct` and `enum` that is used to hold data for
    /// [`logger`](crate::logger) [`behaviors`].
    pub mod types {
        /// [`log_color`](`crate::logger::types::log_color::LogColor`) contains all the [`types`](crate::logger::types)
        /// for log colors.
        pub mod log_color;
        /// [`log_info`](`crate::logger::types::log_info::LogInfo`) contains all the [`types`](crate::logger::types)
        /// for log information.
        pub mod log_info;
        /// [`log_severity`](crate::logger::types::log_severity::LogSeverity) contains all the
        /// [`types`](crate::logger::types) for log severity levels.
        pub mod log_severity;
        /// [`logger`](crate::logger::types::logger::Logger) contains all the [`types`](crate::logger::types) for logging.
        pub mod logger;
    }

    /// [`behaviors`] is a collection of implementations for each [`type`](crate::logger::types).
    pub mod behaviors {
        /// [`log_color`](crate::logger::types::log_color::LogColor) contains all the
        /// [`behaviors`](crate::logger::behaviors) for log colors.
        pub mod log_color;
        /// [`log_info`](crate::logger::types::log_info::LogInfo) contains all the
        /// [`behaviors`](crate::logger::behaviors) for log information.
        pub mod log_info;
        /// [`log_severity`](crate::logger::types::log_severity::LogSeverity) contains all the
        /// [`behaviors`](crate::logger::behaviors) for log severity levels.
        pub mod log_severity;
        /// [`logger`](crate::logger::types::logger::Logger) contains all the
        /// [`behaviors`](crate::logger::behaviors) for logging.
        pub mod logger;
    }

    /// [`macros`] is a collection of macros that are used for [`logger`](crate::logger).
    pub mod macros {
        /// [`log`] contains all the `macro_rules` for [`logger`](crate::logger).
        pub mod log;
    }
}
