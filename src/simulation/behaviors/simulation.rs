use crate::{
    quantum::types::{quantum_gate::QuantumGate, qubit::Qubit, qubit_register::QubitRegister},
    simulation::types::simulation::Simulation,
};

/// Implement the [`Simulation<T>`] trait for [`Qubit`].
impl Simulation<Qubit> for Qubit {
    fn simulate_superposition(&self, amount: i32) -> Vec<bool> {
        (0..amount)
            .map(|_| self.apply_gate(&QuantumGate::SUPERPOSITION).measure())
            .collect()
    }
}

/// Implement the [`Simulation<T>`] trait for [`QubitRegister`].
impl Simulation<QubitRegister> for QubitRegister {
    fn simulate_superposition(&self, amount: i32) -> Vec<bool> {
        (0..self.len())
            .flat_map(|i| {
                (0..amount).map(move |_| {
                    self.get(i)
                        .expect("Qubit invalid")
                        .apply_gate(&QuantumGate::SUPERPOSITION)
                        .measure()
                })
            })
            .collect()
    }
}
