mod individual_stats;
mod population_stats;
mod genetic_stats;

use crate::entities::Individual;
use crate::map::Map;

pub use individual_stats::IndividualStats;
pub use population_stats::PopulationStats;
pub use genetic_stats::GeneticStats;

pub struct SimulationStats {
    pub individual_stats: Vec<IndividualStats>,
    pub population_stats: PopulationStats,
    pub genetic_stats: GeneticStats,
}

impl SimulationStats {
    pub fn new() -> Self {
        SimulationStats {
            individual_stats: Vec::new(),
            population_stats: PopulationStats::new(),
            genetic_stats: GeneticStats::new(),
        }
    }
    pub fn update(&mut self, individuals: &[Individual], map: &Map) {
        self.update_individual_stats(individuals);
        self.population_stats.update(individuals);
        self.genetic_stats.update(individuals);
    }


    fn update_individual_stats(&mut self, individuals: &[Individual]) {
        for individual in individuals {
            if let Some(stats) = self.individual_stats.iter_mut().find(|s| s.id == individual.id) {
                stats.update(individual);
            } else {
                self.individual_stats.push(IndividualStats::new(individual));
            }
        }
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&self.population_stats.generate_report());
        report.push_str(&self.genetic_stats.generate_report());
        report.push_str("\nIndividual Reports:\n");
        for stats in &self.individual_stats {
            report.push_str(&stats.generate_report());
            report.push_str("\n");
        }
        report
    }
}
