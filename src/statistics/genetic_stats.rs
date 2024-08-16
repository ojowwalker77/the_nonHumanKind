use crate::entities::Individual;
use std::collections::HashMap;

pub struct GeneticStats {
    pub gene_frequencies: HashMap<String, u32>,
    pub average_genetic_traits: HashMap<String, f32>,
}

impl GeneticStats {
    pub fn new() -> Self {
        GeneticStats {
            gene_frequencies: HashMap::new(),
            average_genetic_traits: HashMap::new(),
        }
    }

    pub fn update(&mut self, individuals: &[Individual]) {
        self.gene_frequencies.clear();
        self.average_genetic_traits.clear();

        for individual in individuals {
            for (i, _gene) in individual.dna.chars().enumerate() {
                *self.gene_frequencies.entry(format!("Gene_{}", i)).or_insert(0) += 1;
            }

            self.update_average_trait("HP Capacity", individual.genetics.hp_capacity as f32);
            self.update_average_trait("Energy Capacity", individual.genetics.energy_capacity as f32);
            self.update_average_trait("Movement Speed", individual.genetics.movement_speed as f32);
            self.update_average_trait("Natural Energy Loss", individual.genetics.natural_energy_loss as f32);
            self.update_average_trait("Active Energy Loss", individual.genetics.active_energy_loss as f32);
        }

        let population = individuals.len() as f32;
        for value in self.average_genetic_traits.values_mut() {
            *value /= population;
        }
    }

    fn update_average_trait(&mut self, trait_name: &str, value: f32) {
        *self.average_genetic_traits.entry(trait_name.to_string()).or_insert(0.0) += value;
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::from("Genetic Stats:\n");
        report.push_str("Gene Frequencies:\n");
        for (gene, frequency) in &self.gene_frequencies {
            report.push_str(&format!("  {}: {}\n", gene, frequency));
        }
        report.push_str("Average Genetic Traits:\n");
        for (trait_name, average) in &self.average_genetic_traits {
            report.push_str(&format!("  {}: {:.2}\n", trait_name, average));
        }
        report
    }
}
