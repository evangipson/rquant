use rquant::{
    quantum::types::{qubit::Qubit, qubit_register::QubitRegister},
    simulation::types::{simulation::Simulation, simulation_report::SimulationReport},
};

fn main() {
    let qubit_simulations = 10000;

    Qubit::one()
        .simulate_superposition(qubit_simulations)
        .report(Qubit::one());
    Qubit::zero()
        .simulate_superposition(qubit_simulations)
        .report(Qubit::zero());
    Qubit::quarter_turn()
        .simulate_superposition(qubit_simulations)
        .report(Qubit::quarter_turn());
    Qubit::flip()
        .simulate_superposition(qubit_simulations)
        .report(Qubit::flip());

    QubitRegister::new(5)
        .simulate_superposition(qubit_simulations)
        .report(QubitRegister::new(5));
}
