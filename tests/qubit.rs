#[cfg(test)]
mod tests {
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
        let expected = KET_ZERO;
        let qubit = Qubit::new(QuantumPosition::new(expected, KET_ONE));

        let result = qubit.initial_position();

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
        let expected = KET_ZERO;
        let qubit = Qubit::new(QuantumPosition::new(KET_ONE, expected));

        let result = qubit.possible_position();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn measure_shouldpanic_withoutanypositions() {
        let qubit = Qubit { positions: vec![] };

        qubit.measure();
    }

    #[test]
    #[should_panic]
    fn applygate_shouldpanic_withoutanypositions() {
        let qubit = Qubit { positions: vec![] };

        qubit.apply_gate(&QuantumGate::NOT);
    }

    #[test]
    fn applygate_shouldflipqubit_withnotgate() {
        let qubit = Qubit::zero();

        let result = qubit.apply_gate(&QuantumGate::NOT);

        assert_eq!(Qubit::one(), result);
    }

    #[test]
    fn applygate_shouldphaseonequbit_withphasegate() {
        let qubit = Qubit::one();

        let result = qubit.apply_gate(&QuantumGate::PHASE);

        assert_eq!(Qubit::flip(), result);
    }

    #[test]
    fn applygate_shouldnotphasezeroqubit_withphasegate() {
        let qubit = Qubit::zero();

        let result = qubit.apply_gate(&QuantumGate::PHASE);

        assert_eq!(Qubit::zero(), result);
    }

    #[test]
    fn applygate_shouldrotateonequbit_withrotategate() {
        //  -i|0>
        // ┏    ┓
        // ┃ -i ┃
        // ┃  0 ┃
        // ┗    ┛
        let flipped_position = QuantumPosition::new(KET_BACK_ROTATION, KET_ZERO);
        let expected = Qubit::new(flipped_position);
        //  |1>
        // ┏   ┓
        // ┃ 0 ┃
        // ┃ 1 ┃
        // ┗   ┛
        let qubit = Qubit::one();

        //  rotate   |1>  =     apply_gate     =  -i|0>
        // ┏      ┓ ┏   ┓   ┏                ┓   ┏    ┓
        // ┃ 0 -i ┃ ┃ 0 ┃ = ┃ (0*0) + (-i*1) ┃ = ┃ -i ┃
        // ┃ i  0 ┃ ┃ 1 ┃   ┃ (i*0) +  (0*1) ┃   ┃  0 ┃
        // ┗      ┛ ┗   ┛   ┗                ┛   ┗    ┛
        let result = qubit.apply_gate(&QuantumGate::ROTATE);

        assert_eq!(expected, result);
    }

    #[test]
    fn notoperator_shouldflipqubit() {
        let qubit = Qubit::zero();

        let result = !qubit;

        assert_eq!(Qubit::one(), result);
    }
}
