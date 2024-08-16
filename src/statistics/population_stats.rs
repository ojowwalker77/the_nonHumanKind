use crate::entities::Individual;

pub struct PopulationStats {
    pub total_population: u32,
    pub average_energy: f32,
    pub average_hp: f32,
    pub births: u32,
    pub deaths: u32,
}

impl PopulationStats {
    pub fn new() -> Self {
        PopulationStats {
            total_population: 0,
            average_energy: 0.0,
            average_hp: 0.0,
            births: 0,
            deaths: 0,
        }
    }

    pub fn update(&mut self, individuals: &[Individual]) {
        self.total_population = individuals.len() as u32;
        self.average_energy = individuals.iter().map(|i| i.energy as f32).sum::<f32>() / self.total_population as f32;
        self.average_hp = individuals.iter().map(|i| i.hp as f32).sum::<f32>() / self.total_population as f32;
        // Births and deaths should be updated elsewhere when those events occur
    }

    pub fn generate_report(&self) -> String {
        format!(
            "Population Stats:\nTotal: {}, Avg Energy: {:.2}, Avg HP: {:.2}, Births: {}, Deaths: {}\n",
            self.total_population, self.average_energy, self.average_hp, self.births, self.deaths
        )
    }
}
