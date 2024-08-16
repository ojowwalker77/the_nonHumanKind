use super::Plant;

pub struct FruitTree {
    fruits: u32,
    growth_counter: u32,
}

impl FruitTree {
    pub fn new() -> Self {
        FruitTree {
            fruits: 1,
            growth_counter: 0,
        }
    }
}

impl Plant for FruitTree {
    fn update(&mut self) {
        if self.fruits == 0 {
            self.growth_counter += 1;
            if self.growth_counter >= 10 {
                self.fruits = 1;
                self.growth_counter = 0;
            }
        }
    }

    fn can_be_eaten(&self) -> bool {
        self.fruits > 0
    }

    fn eat(&mut self) -> u32 {
        if self.fruits > 0 {
            self.fruits -= 1;
            50
        } else {
            0
        }
    }
}
