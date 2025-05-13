use crate::{
    log_info, quantum::types::qubit::Qubit, simulation::types::simulation_report::SimulationReport,
};

/// Implement the [`SimulationReport<Qubit>`] trait for [`Vec<T>`] of [`bool`].
impl SimulationReport<Qubit> for Vec<bool> {
    fn report(&self, report_for: Qubit) {
        let total = self.len() as f64;
        let true_count = self.iter().filter(|&p| *p).count() as f64;
        let false_count = self.iter().filter(|&p| !*p).count() as f64;
        log_info!(
            "Simulation report results for {}\n  true  :  {} ({:.2}%)\n  false :  {} ({:.2}%)\n  total : {}",
            report_for,
            true_count as u32,
            (true_count / total) * 100.0,
            false_count as u32,
            (false_count / total) * 100.0,
            self.len()
        );
    }
}
