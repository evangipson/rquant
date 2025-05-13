/// [`SimulationReport<T>`] is a [`trait`] that allows reporting for any type that implements it.
pub trait SimulationReport<T> {
    /// [`SimulationReport<T>::report`] will log out results, normally of an implementation of some
    /// [`Simulation<T>`](crate::simulation::types::simulation).
    fn report(&self, report_for: T);
}
