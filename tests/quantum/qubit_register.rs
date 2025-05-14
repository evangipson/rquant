use rquant::quantum::types::{
    quantum_gate::QuantumGate, qubit::Qubit, qubit_register::QubitRegister,
};

#[test]
fn new_shouldcreateblankregister_withzeroqubits() {
    let qubit_register = QubitRegister::new(0);

    assert_eq!(0, qubit_register.len());
}

#[test]
fn new_shouldcreateregister_withmultiplequbits() {
    let qubit_register = QubitRegister::new(5);

    assert!(!qubit_register.is_empty());
}

#[test]
fn len_shouldreturnzero_withoutqubits() {
    let qubit_register = QubitRegister { qubits: vec![] };

    assert_eq!(0, qubit_register.len());
}

#[test]
fn len_shouldreturnqubitamount() {
    let num_qubits = 5;
    let qubit_register = QubitRegister::new(num_qubits);

    assert_eq!(num_qubits, qubit_register.len());
}

#[test]
fn isempty_shouldreturntrue_foremptyregister() {
    let qubit_register = QubitRegister { qubits: vec![] };

    assert!(qubit_register.is_empty());
}

#[test]
fn get_shouldreturnnone_withinvalidindex() {
    let qubit_register = QubitRegister::new(0);

    assert!(qubit_register.get(999).is_none());
}

#[test]
fn get_shouldreturnsome_withvalidindex() {
    let qubit_register = QubitRegister::new(5);

    assert!(qubit_register.get(0).is_some());
}

#[test]
fn getmut_shouldreturnnone_withinvalidindex() {
    let mut qubit_register = QubitRegister::new(0);

    assert!(qubit_register.get_mut(999).is_none());
}

#[test]
fn getmut_shouldreturnsome_withvalidindex() {
    let mut qubit_register = QubitRegister::new(5);

    assert!(qubit_register.get_mut(0).is_some());
}

#[test]
fn applysinglequbitgate_shouldmodifyregister_withvalidgate() {
    let expected = Qubit::zero()
        .apply_gate(&QuantumGate::SUPERPOSITION)
        .initial_position();
    let qubit_to_modify_index = 0;
    let mut qubit_register = QubitRegister::new(10);

    qubit_register.apply_single_qubit_gate(&QuantumGate::SUPERPOSITION, qubit_to_modify_index);
    let modified_qubit = qubit_register
        .get_mut(qubit_to_modify_index)
        .expect("Unable to get superpositioned qubit in register.");

    assert_eq!(expected, modified_qubit.initial_position());
}
