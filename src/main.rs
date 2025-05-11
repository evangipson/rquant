use rquant::types::qubit::Qubit;

fn main() {
    println!("==========\nZero qubit\n=========={}\n", Qubit::zero());
    println!("=========\nOne qubit\n========={}\n", Qubit::one());

    println!(
        "========================\nZero Qubit with NOT gate\n========================{}\n",
        !Qubit::zero()
    );
    println!(
        "=======================\nOne Qubit with NOT gate\n======================={}",
        !Qubit::one()
    );
}
