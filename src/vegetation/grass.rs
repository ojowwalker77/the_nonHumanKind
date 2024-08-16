use super::Plant;

pub struct Grass {
    energy: u32,
    growth_rate: u32,
}

impl Grass {
    pub fn new() -> Self {
        Grass {
            energy: 1,
            growth_rate: 1,
        }
    }
}

impl Plant for Grass {
    fn update(&mut self) {
        self.energy = (self.energy + self.growth_rate).min(10);
    }

    fn can_be_eaten(&self) -> bool {
        self.energy > 0
    }

    fn eat(&mut self) -> u32 {
        let energy = self.energy;
        self.energy = 0;
        energy
    }
}
