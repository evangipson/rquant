use rquant::quantum::{
    constants::ket::{KET_BACK_ROTATION, KET_ONE, KET_ZERO},
    types::{quantum_gate::QuantumGate, quantum_position::QuantumPosition, qubit::Qubit},
};

#[test]
fn new_shouldmakequbit_withvalidposition() {
    let expected_initial_position = KET_ONE;
    let expected_possible_position = KET_ZERO;

    let result = Qubit::new(QuantumPosition::new(
        expected_initial_position,
        expected_possible_position,
    ));

    assert_eq!(expected_initial_position, result.initial_position());
    assert_eq!(expected_possible_position, result.possible_position());
}

#[test]
#[should_panic]
fn initialposition_shouldpanic_withoutanypositions() {
    let qubit = Qubit { positions: vec![] };

    qubit.initial_position();
}

#[test]
fn initialposition_shouldreturnposition_withvalidqubit() {
    let expected = QuantumPosition::ONE.initial_position;
    let result = Qubit::new(QuantumPosition::ONE).initial_position();

    assert_eq!(expected, result);
}

#[test]
#[should_panic]
fn possibleposition_shouldpanic_withoutanypositions() {
    let qubit = Qubit { positions: vec![] };

    qubit.possible_position();
}

#[test]
fn possibleposition_shouldreturnposition_withvalidqubit() {
    let expected = QuantumPosition::ONE.possible_position;
    let result = Qubit::new(QuantumPosition::ONE).possible_position();

    assert_eq!(expected, result);
}

#[test]
fn update_shouldaddposition_withoutanypositions() {
    let mut qubit = Qubit { positions: vec![] };

    qubit.update(QuantumPosition::ONE);

    assert_eq!(1, qubit.positions.len());
    assert_eq!(
        QuantumPosition::ONE.initial_position,
        qubit.initial_position()
    );
    assert_eq!(
        QuantumPosition::ONE.possible_position,
        qubit.possible_position()
    );
}

#[test]
fn update_shouldaddposition_toexistingpositions() {
    let mut qubit = Qubit::one();

    qubit.update(QuantumPosition::ONE);

    assert_eq!(2, qubit.positions.len());
    assert_eq!(
        QuantumPosition::ONE.initial_position,
        qubit.initial_position()
    );
    assert_eq!(
        QuantumPosition::ONE.possible_position,
        qubit.possible_position()
    );
}

#[test]
#[should_panic]
fn measure_shouldpanic_withoutanypositions() {
    let qubit = Qubit { positions: vec![] };

    qubit.measure();
}

#[test]
fn measure_shouldreturnfalse_foridentityqubit() {
    assert!(!Qubit::one().measure());
}

#[test]
fn measure_shouldreturntrue_forzeroqubit() {
    assert!(Qubit::zero().measure());
}

#[test]
#[should_panic]
fn applygate_shouldpanic_withoutanypositions() {
    let qubit = Qubit { positions: vec![] };

    qubit.apply_gate(&QuantumGate::NOT);
}

#[test]
fn applygate_shouldflipqubit_withnotgate() {
    let result = Qubit::zero().apply_gate(&QuantumGate::NOT);

    assert_eq!(Qubit::one(), result);
}

#[test]
fn applygate_shouldphaseonequbit_withphasegate() {
    let result = Qubit::one().apply_gate(&QuantumGate::PHASE);

    assert_eq!(Qubit::flip(), result);
}

#[test]
fn applygate_shouldnotphasezeroqubit_withphasegate() {
    let result = Qubit::zero().apply_gate(&QuantumGate::PHASE);

    assert_eq!(Qubit::zero(), result);
}

#[test]
fn applygate_shouldrotateonequbit_withrotategate() {
    let expected = Qubit::new(QuantumPosition::new(KET_BACK_ROTATION, KET_ZERO));

    let result = Qubit::one().apply_gate(&QuantumGate::ROTATE);

    assert_eq!(expected, result);
}

#[test]
fn notoperator_shouldflipqubit() {
    let qubit = Qubit::zero();

    let result = !qubit;

    assert_eq!(Qubit::one(), result);
}
