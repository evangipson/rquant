/// [`setup`] contains common functionality for each test.
pub fn setup() {
    // suppress the stack trace output for tests that should fail.
    std::panic::set_hook(Box::new(|_| {}));
}

#[cfg(test)]
mod quantum {
    mod quantum_gate;
    mod quantum_position;
    mod qubit;
    mod qubit_register;
}
