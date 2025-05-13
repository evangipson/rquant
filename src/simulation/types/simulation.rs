/// [`Simulation<T>`] is a [`trait`] that will allow any generic type
/// to simulate behaviors.
pub trait Simulation<T> {
    /// [`Simulation<T>::simulate_superposition`] will simulate
    /// superposition an `amount` of times.
    fn simulate_superposition(&self, amount: i32) -> Vec<bool>;
}
