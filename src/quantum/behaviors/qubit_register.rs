use crate::quantum::types::{
    quantum_gate::QuantumGate, qubit::Qubit, qubit_register::QubitRegister,
};

impl QubitRegister {
    /// [`QubitRegister::new`] Creates a new [`QubitRegister`] with a specified number
    /// of [`Qubits`](crate::quantum::types::qubit::Qubit), all initialized as [`Qubit::zero`].
    ///
    /// # Example
    /// [`QubitRegister::new`] can be used to create a new [`QubitRegister`].
    /// ```rust
    /// use rquant::quantum::types::{qubit_register::QubitRegister, qubit::Qubit};
    ///
    /// fn create_qubit_register(amount: usize) -> QubitRegister {
    ///     QubitRegister::new(amount)
    /// }
    /// ```
    pub fn new(num_qubits: usize) -> Self {
        QubitRegister {
            qubits: (0..num_qubits).map(|_| Qubit::zero()).collect(),
        }
    }

    /// [`QubitRegister::len`] returns the number of qubits in the register.
    ///
    /// # Example
    /// [`QubitRegister::len`] can be used to return the number of
    /// [`Qubits`](crate::quantum::types::qubit::Qubit) in a [`QubitRegister`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit_register::QubitRegister, qubit::Qubit};
    ///
    /// fn count_qubits_in_register(qubit_register: QubitRegister) -> usize {
    ///     qubit_register.len()
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// [`QubitRegister::is_empty`] returns `true` if the register has no
    /// [`Qubits`](crate::quantum::types::qubit::Qubit), and `false` otherwise.
    ///
    /// # Example
    /// [`QubitRegister::is_empty`] can be used to determine if a [`QubitRegister`]
    /// has any [`Qubits`](crate::quantum::types::qubit::Qubit):
    /// ```rust
    /// use rquant::quantum::types::{qubit_register::QubitRegister, qubit::Qubit};
    ///
    /// fn register_has_qubits(qubit_register: QubitRegister) -> bool {
    ///     !qubit_register.is_empty()
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.qubits.len() == 0
    }

    /// [`QubitRegister::get`] gets a reference to a specific qubit in the register,
    /// and returns [`None`] if the `index` is out of bounds.
    ///
    /// # Example
    /// [`QubitRegister::get`] can be used to get a [`Qubit`] from a [`QubitRegister`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit_register::QubitRegister, qubit::Qubit};
    ///
    /// fn get_second_qubit(qubit_register: &QubitRegister) -> &Qubit {
    ///     qubit_register.get(1).expect("Register did not have two qubits.")
    /// }
    /// ```
    pub fn get(&self, index: usize) -> Option<&Qubit> {
        self.qubits.get(index)
    }

    /// [`QubitRegister::get_mut`] gets a mutable reference to a specific qubit in
    /// the register, and returns [`None`] if the `index` is out of bounds.
    ///
    /// # Example
    /// [`QubitRegister::get_mut`] can be used to get a mutable [`Qubit`] from a
    /// [`QubitRegister`]:
    /// ```rust
    /// use rquant::quantum::types::{qubit_register::QubitRegister, qubit::Qubit};
    ///
    /// fn get_second_qubit(qubit_register: &mut QubitRegister) -> &mut Qubit {
    ///     qubit_register.get_mut(1).expect("Register did not have two qubits.")
    /// }
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Qubit> {
        self.qubits.get_mut(index)
    }

    /// [`QubitRegister::apply_single_qubit_gate`] applies a single-qubit [`QuantumGate`]
    /// to a specific [`Qubit`] in a [`QubitRegister`].
    ///
    /// # Example
    /// [`QubitRegister::apply_single_qubit_gate`] can be used to apply a single-qubit
    /// [`QuantumGate`] to a [`Qubit`] in a [`QubitRegister`]:
    /// ```rust
    /// use rquant::quantum::types::{
    ///     quantum_gate::QuantumGate,
    ///     qubit_register::QubitRegister,
    ///     qubit::Qubit
    /// };
    ///
    /// fn flip_second_qubit_in_register(qubit_register: &mut QubitRegister) {
    ///     qubit_register.apply_single_qubit_gate(&QuantumGate::NOT, 1)
    /// }
    /// ```
    pub fn apply_single_qubit_gate(&mut self, gate: &QuantumGate, target_qubit: usize) {
        if let Some(qubit) = self.qubits.get_mut(target_qubit) {
            *qubit = qubit.apply_gate(gate);
        } else {
            eprintln!("Error: Invalid qubit index");
        }
    }
}

/// Implement the [`std::fmt::Display`] trait for [`QubitRegister`].
impl std::fmt::Display for QubitRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        for (i, qubit) in self.qubits.iter().enumerate() {
            write!(f, "{}", qubit)?;
            if i < self.qubits.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ">")
    }
}
