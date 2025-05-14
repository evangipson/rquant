# rquant
![rustc (with version)](https://img.shields.io/badge/rustc-1.56.0+-blue?style=for-the-badge&logo=rust) ![crate.rs (with version)](https://img.shields.io/crates/v/rquant?style=for-the-badge&logo=hackthebox&logoColor=white) ![docs.rs (with version)](https://img.shields.io/docsrs/rquant/latest?style=for-the-badge&logo=rust)

A quantum computing library written entirely in rust.

It allows for qubit measurement and basic quantum logic in complex vector space. Gates can be applied to qubits in complex vector space, then those qubits can be measured to observe the outcome.

## Getting Started
1. Download the repo
1. Open a terminal and navigate to the repo root
1. Run `cargo run` to see the output of the (very simple) [`main`](src/main.rs) function
1. Run `cargo test` to run all the tests
1. Run `cargo doc --no-deps` to generate the docs

## Examples
### Creating a qubit
You can create a qubit by using one of the premade qubits, or using a premade quantum position:
```rust
use rquant::{
    log_info,
    quantum::types::{quantum_position::QuantumPosition, qubit::Qubit},
};

fn main() {
    // create a zero qubit: |0>
    let zero_qubit = Qubit::zero();
    log_info!("Zero qubit: {zero_qubit}");

    // create an identity qubit: |1>
    let identity_qubit = Qubit::one();
    log_info!("Zero qubit: {identity_qubit}");

    // create a custom qubit using a quantum position:
    // 0+1i|0>
    let custom_qubit = Qubit::new(QuantumPosition::QUARTER_TURN);
    log_info!("Custom qubit: {custom_qubit}");
}
```

This example will produce the following output:

![A screenshot of console output from qubit creation using the rquant rust crate](https://raw.githubusercontent.com/evangipson/rquant/blob/main/assets/first-example-output.png?raw=true)

### Applying gates to qubits
You can apply a gate to a single qubit:
```rust
use rquant::{
    log_info,
    quantum::types::{quantum_gate::QuantumGate, qubit::Qubit},
};

fn main() {
    // use the '!' operator to apply the FLIP gate
    let flipped_zero_qubit = !Qubit::zero();
    log_info!("Flipped zero qubit: {flipped_zero_qubit}");

    // use premade quantum gates to modify a qubit
    let rotated_qubit = Qubit::one()
        .apply_gate(&QuantumGate::PHASE)
        .apply_gate(&QuantumGate::ROTATE);
    log_info!("Rotated identity qubit: {rotated_qubit}");
}
```

This example will produce the following output:

![A screenshot of console output from applying a gate to a qubit using the rquant rust crate](https://raw.githubusercontent.com/evangipson/rquant/blob/main/assets/second-example-output.png?raw=true)

### Create multiple associated qubits
You can create a collection of many qubits using a qubit register:
```rust
use rquant::{
    log_info,
    quantum::types::{quantum_gate::QuantumGate, qubit_register::QubitRegister},
};

fn main() {
    // create a register of 100 qubits with the |0> state
    let qubit_register = QubitRegister::new(10);
    log_info!("Qubit register:\n{qubit_register}");

    // get the amount of qubits in the qubit register
    let qubit_count = qubit_register.len();
    log_info!("There are {qubit_count} qubits in the register.");

    // get a (zero-indexed) qubit from the register
    let fifth_qubit = qubit_register
        .get(4)
        .expect("There was no fifth qubit in the register.");
    log_info!("The fifth qubit in the register: {fifth_qubit}");

    // modify the fifth qubit by applying the PHASE gate
    let phased_fifth_qubit = fifth_qubit.apply_gate(&QuantumGate::ROTATE);
    log_info!("Phased fifth qubit: {phased_fifth_qubit}\n");
}
```

This example will produce the following output:

![A screenshot of console output from creating a qubit register using the rquant rust crate](https://raw.githubusercontent.com/evangipson/rquant/blob/main/assets/third-example-output.png?raw=true)

### Run simulations on qubits
You can simulate and report on behavior on any amount of qubits:
```rust
use rquant::{
    quantum::types::qubit::Qubit,
    simulation::types::{simulation::Simulation, simulation_report::SimulationReport},
};

fn main() {
    // create 1,000,000 superpositioned qubits, and observe
    // their measurements using the report function.
    Qubit::one()
        .simulate_superposition(1000000)
        .report(Qubit::one());
}
```

This example will produce the following output:

![A screenshot of console output from qubit simulation using the rquant rust crate](https://raw.githubusercontent.com/evangipson/rquant/blob/main/assets/fourth-example-output.png?raw=true)

## Dependencies
|Crate|Purpose|
|-|-|
|[rand v0.9.1](https://docs.rs/rand/0.9.1/rand/index.html)|Used to measure qubit position|
|[num-complex v0.4](https://docs.rs/num-complex/0.4.6/num_complex/index.html)|Used as the basis of quantum positions for qubits|