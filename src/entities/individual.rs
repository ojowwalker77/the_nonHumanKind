use crate::map::Coordinate;
use crate::entities::Genetics;

pub struct Individual {
    pub id: u64,
    pub position: Coordinate,
    pub hp: u32,
    pub energy: u32,
    pub dna: String,
    pub genetics: Genetics,
}

impl Individual {
    pub fn new(id: u64, position: Coordinate, dna: String) -> Result<Self, &'static str> {
        let genetics = Genetics::from_dna(&dna)?;
        Ok(Individual {
            id,
            position,
            hp: genetics.hp_capacity,
            energy: genetics.energy_capacity,
            dna,
            genetics,
        })
    }

    pub fn update(&mut self) {
        self.energy = self.energy.saturating_sub(self.genetics.natural_energy_loss);
        if self.energy < self.genetics.energy_capacity / 3 {
            self.hp = self.hp.saturating_sub(1);
        }
    }

    pub fn move_to(&mut self, new_position: Coordinate) {
        self.position = new_position;
        self.energy = self.energy.saturating_sub(self.genetics.active_energy_loss);
    }

    pub fn needs_food(&self) -> bool {
        self.energy < self.genetics.energy_capacity * 6 / 10
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

           let mut individual = Self::new(id, Coordinate::new(x, y), dna)?;
           individual.hp = hp;
           individual.energy = energy;

           Ok(individual)
       }
   }
