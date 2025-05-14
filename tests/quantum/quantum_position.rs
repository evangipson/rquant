use num_complex::Complex;
use rquant::quantum::types::quantum_position::QuantumPosition;

#[test]
fn new_shouldcreatequantumposition() {
    let quantum_position = QuantumPosition::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));

    assert!(quantum_position.has_valid_amplitude());
}

#[test]
fn hasvalidposition_shouldbefalse_withinvalidamplitude() {
    // i|i> is not a valid amplitude
    let quantum_position = QuantumPosition::new(Complex::new(0.0, 1.0), Complex::new(0.0, 1.0));

    assert!(!quantum_position.has_valid_amplitude());
}
