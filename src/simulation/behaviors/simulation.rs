use crate::{
    quantum::types::{quantum_gate::QuantumGate, qubit::Qubit},
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
