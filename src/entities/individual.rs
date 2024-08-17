use crate::map::{Map, Coordinate};
use rand::Rng;
use super::genetics::Genetics;

pub struct Individual {
    pub id: u64,
    pub position: Coordinate,
    pub hp: u32,
    pub energy: u32,
    pub dna: String,
    pub genetics: Genetics,
    pub stats: IndividualStats,
}

pub struct IndividualStats {
    pub food_consumed: u32,
    pub total_distance: u32,
}

impl Individual {
    pub fn new(id: u64, position: Coordinate, dna: Option<String>) -> Result<Self, &'static str> {
        let dna = match dna {
            Some(d) => d,
            None => {
                let mut rng = rand::thread_rng();
                (0..10).map(|_| rng.gen_range(0..10).to_string()).collect()
            }
        };
        let genetics = Genetics::from_dna(&dna)?;
        Ok(Individual {
            id,
            position,
            hp: genetics.hp_capacity,
            energy: genetics.energy_capacity,
            dna,
            genetics,
            stats: IndividualStats {
                food_consumed: 0,
                total_distance: 0,
            },
        })
    }

    pub fn update(&mut self) {
        // Reduce natural energy loss
        self.energy = self.energy.saturating_sub(self.genetics.natural_energy_loss / 10);

        // Regenerate HP if energy is above 50%
        if self.energy > self.genetics.energy_capacity / 2 && self.hp < self.genetics.hp_capacity {
            self.hp = (self.hp + 1).min(self.genetics.hp_capacity);
        }

        // Lose HP if energy is 0
        if self.energy == 0 {
            self.hp = self.hp.saturating_sub(1);
        }
    }

    pub fn move_to(&mut self, new_position: Coordinate) {
        self.position = new_position;
        self.energy = self.energy.saturating_sub(self.genetics.active_energy_loss / 5);
        self.stats.total_distance += 1;
    }

    pub fn needs_food(&self) -> bool {
        self.energy < self.genetics.energy_capacity * 6 / 10
    }

    pub fn eat(&mut self, map: &mut Map) {
        if let Some(energy_gained) = map.eat_plant_at(&self.position) {
            let adjusted_energy = energy_gained * self.genetics.food_efficiency;
            self.energy = (self.energy + adjusted_energy).min(self.genetics.energy_capacity);
            self.stats.food_consumed += 1;
        }
    }

     pub fn mutate(&mut self, rng: &mut impl Rng) {
         let mut new_dna = String::new();
         for gene in self.dna.chars() {
             if rng.gen::<f32>() < 0.1 {  // 10% chance of mutation per gene
                 new_dna.push(char::from_digit(rng.gen_range(0..10), 10).unwrap());
             } else {
                 new_dna.push(gene);
             }
         }
         self.dna = new_dna;
         self.genetics = Genetics::from_dna(&self.dna).unwrap();
     }

    pub fn to_state_string(&self) -> String {
           format!(
               "ID:{:05};POS:{:04},{:04};HP:{:03};EN:{:03};DNA:{}",
               self.id,
               self.position.x,
               self.position.y,
               self.hp,
               self.energy,
               self.dna
           )
       }

       pub fn from_state_string(state: &str) -> Result<Self, &'static str> {
           let parts: Vec<&str> = state.split(';').collect();
           if parts.len() != 5 {
               return Err("Invalid state string format");
           }

           let id = parts[0][3..].parse::<u64>().map_err(|_| "Invalid ID")?;
           let pos: Vec<&str> = parts[1][4..].split(',').collect();
           if pos.len() != 2 {
               return Err("Invalid position format");
           }
           let x = pos[0].parse::<usize>().map_err(|_| "Invalid X coordinate")?;
           let y = pos[1].parse::<usize>().map_err(|_| "Invalid Y coordinate")?;
           let hp = parts[2][3..].parse::<u32>().map_err(|_| "Invalid HP")?;
           let energy = parts[3][3..].parse::<u32>().map_err(|_| "Invalid energy")?;
           let dna = parts[4][4..].to_string();

           let mut individual = Self::new(id, Coordinate::new(x, y), Some(dna))?;
           individual.hp = hp;
           individual.energy = energy;

           Ok(individual)
       }
   }
