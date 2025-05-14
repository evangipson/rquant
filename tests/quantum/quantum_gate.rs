use rquant::quantum::types::{quantum_gate::QuantumGate, quantum_operators::QuantumOperator};

#[test]
fn new_shouldmakequantumgate_withvalidoperator() {
    [
        QuantumOperator::NOT,
        QuantumOperator::PHASE,
        QuantumOperator::ROTATE,
        QuantumOperator::SUPERPOSITION,
    ]
    .iter()
    .for_each(|op| {
        let quantum_gate = QuantumGate::new(op.clone());
        assert_eq!(*op, quantum_gate.operator);
        assert!(!quantum_gate.transform.is_empty());
    });
}
