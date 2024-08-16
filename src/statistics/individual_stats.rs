use crate::entities::Individual;

pub struct IndividualStats {
    pub id: u64,
    pub lifetime: u32,
    pub max_energy: u32,
    pub min_energy: u32,
    pub total_distance: u32,
    pub food_consumed: u32,
}

impl IndividualStats {
    pub fn new(individual: &Individual) -> Self {
        IndividualStats {
            id: individual.id,
            lifetime: 0,
            max_energy: individual.energy,
            min_energy: individual.energy,
            total_distance: 0,
            food_consumed: 0,
        }
    }

    pub fn update(&mut self, individual: &Individual) {
        self.lifetime += 1;
        self.max_energy = self.max_energy.max(individual.energy);
        self.min_energy = self.min_energy.min(individual.energy);
        // Assuming you have a way to track distance and food consumption
        // self.total_distance += ...;
        // self.food_consumed += ...;
    }

    pub fn generate_report(&self) -> String {
        format!(
            "Individual {}: Lifetime: {}, Max Energy: {}, Min Energy: {}, Total Distance: {}, Food Consumed: {}",
            self.id, self.lifetime, self.max_energy, self.min_energy, self.total_distance, self.food_consumed
        )
    }
}
