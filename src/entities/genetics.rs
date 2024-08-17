pub struct Genetics {
    pub hp_capacity: u32,
    pub energy_capacity: u32,
    pub movement_speed: u32,
    pub natural_energy_loss: u32,
    pub active_energy_loss: u32,
    pub food_efficiency: u32,
    pub reproduction_rate: u32,
    pub mutation_rate: u32,
}

impl Genetics {
    pub fn from_dna(dna: &str) -> Result<Self, &'static str> {
        if dna.len() != 10 || !dna.chars().all(|c| c.is_digit(10)) {
            return Err("Invalid DNA string");
        }

        Ok(Genetics {
            hp_capacity: dna[0..1].parse::<u32>().unwrap() * 20 + 50, // 50-230
            energy_capacity: dna[1..2].parse::<u32>().unwrap() * 30 + 50, // 50-320
            movement_speed: dna[2..3].parse::<u32>().unwrap() + 1, // 1-10
            natural_energy_loss: dna[3..5].parse::<u32>().unwrap() / 5 + 1, // 1-20
            active_energy_loss: dna[5..7].parse::<u32>().unwrap() / 2 + 5, // 5-54
            food_efficiency: dna[7..8].parse::<u32>().unwrap() * 2 + 1, // 1-19
            reproduction_rate: dna[8..9].parse::<u32>().unwrap(), // 0-9
            mutation_rate: dna[9..10].parse::<u32>().unwrap(), // 0-9
        })
    }
}
