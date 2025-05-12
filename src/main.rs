use rquant::{
    log_info,
    quantum::types::{quantum_gate::QuantumGate, qubit::Qubit},
};

fn get_qubit_variations(name: &str, qubit: Qubit) -> String {
    let flipped_qubit = !qubit.clone();
    let phased_qubit = qubit.apply_gate(&QuantumGate::PHASE);
    let rotated_qubit = qubit.apply_gate(&QuantumGate::ROTATE);
    format!("{name}\t{qubit}\t{flipped_qubit}\t{phased_qubit}\t{rotated_qubit}")
}

fn get_qubit_evaluations(name: &str, qubit: Qubit) -> String {
    let qubit_eval = qubit.measure();
    let flipped_qubit_eval = !qubit.clone().measure();
    let phased_qubit_eval = qubit.apply_gate(&QuantumGate::PHASE).measure();
    let rotated_qubit_eval = qubit.apply_gate(&QuantumGate::ROTATE).measure();
    format!("{name}\t{qubit_eval}\t{flipped_qubit_eval}\t{phased_qubit_eval}\t{rotated_qubit_eval}")
}

fn main() {
    log_info!(
        "\n{:^39}\n\nName\tQubit\tNOT\tPHASE\tROTATE\n{:-<39}\n{}\n{}",
        "Qubit Notations",
        "",
        get_qubit_variations("Zero", Qubit::zero()),
        get_qubit_variations("One", Qubit::one()),
    );

    log_info!(
        "\n{:^39}\n\nName\tQubit\tNOT\tPHASE\tROTATE\n{:-<39}\n{}\n{}\n",
        "Qubit Evaluations",
        "",
        get_qubit_evaluations("Zero", Qubit::zero()),
        get_qubit_evaluations("One", Qubit::one()),
    );
}
