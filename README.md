# rquant
A quantum computing library written entirely in rust.

It allows for qubit measurement and basic quantum logic in complex vector space. Gates can be applied to qubits in complex vector space, then those qubits can be measured to observe the outcome.

The list of currently supported gates is as follows:

|Gate|Transform|Outcome|
|-|:-:|:-:|
|**NOT**|$\begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}$|$\begin{pmatrix} 0 & 1 \\\ 1 & 0 \end{pmatrix}\begin{pmatrix} \alpha \\\ \beta \end{pmatrix} = \begin{pmatrix} \beta \\\ \alpha \end{pmatrix}$|
|**FLIP**|$\begin{pmatrix} 0 & -i \\\ i & 0 \end{pmatrix}$| $\begin{pmatrix} 0 & -i \\\ i & 0 \end{pmatrix} \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} = \begin{pmatrix} -i \beta \\\ i \alpha \end{pmatrix}$|
|**PHASE**|$\begin{pmatrix} 1 & 0 \\\ 0 & -1 \end{pmatrix}$| $\begin{pmatrix} 1 & 0 \\\ 0 & -1 \end{pmatrix} \begin{pmatrix} \alpha \\\ \beta \end{pmatrix} = \begin{pmatrix} \alpha \\\ -\beta \end{pmatrix}$|

## Getting Started
1. Download the repo
1. Open a terminal and navigate to the repo root
1. Run `cargo run` to see the output of the (very simple) [`main`](./src/main.rs) function
1. Run `cargo test --doc` to run all the docs tests
1. Run `cargo doc` to generate the docs

## Examples
You can do basic operations on a qubit like so:
```rust
use rquant::types::qubit::Qubit;
use rquant::types::quantum_gate::QuantumGate;

fn main() {
    let zero_qubit = Qubit::zero();
    println!("Zero Qubit:{}", zero_qubit);

    let not_zero_qubit = !zero_qubit;
    println!("Zero Qubit with NOT gate applied:{}", zero_qubit);

    let one_qubit = Qubit::one();
    println!("One Qubit:{}", zero_qubit);

    let flipped_one_qubit = one_qubit.apply_gate(&QuantumGate::FLIP);
    println!("One Qubit with FLIP gate applied:{}", flipped_one_qubit);
}
```

## Dependencies
|Crate|Purpose|
|-|-|
|[rand v0.9.1](https://docs.rs/rand/0.9.1/rand/index.html)|Used to measure qubit position|
|[num-complex v0.4](https://docs.rs/num-complex/0.4.6/num_complex/index.html)|Used as the basis of quantum positions for qubits|