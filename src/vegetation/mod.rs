mod plant;
mod grass;
mod fruit_tree;

pub use plant::Plant;
pub use grass::Grass;
pub use fruit_tree::FruitTree;

use crate::map::Coordinate;

pub enum VegetationType {
    Grass(Grass),
    FruitTree(FruitTree),
}

impl VegetationType {
    pub fn update(&mut self) {
        match self {
            VegetationType::Grass(grass) => grass.update(),
            VegetationType::FruitTree(tree) => tree.update(),
        }
    }

    pub fn can_be_eaten(&self) -> bool {
        match self {
            VegetationType::Grass(grass) => grass.can_be_eaten(),
            VegetationType::FruitTree(tree) => tree.can_be_eaten(),
        }
    }

    pub fn eat(&mut self) -> u32 {
        match self {
            VegetationType::Grass(grass) => grass.eat(),
            VegetationType::FruitTree(tree) => tree.eat(),
        }
    }
}

pub struct VegetationSystem {
    pub plants: Vec<(Coordinate, VegetationType)>,
}

impl VegetationSystem {
    pub fn new() -> Self {
        VegetationSystem { plants: Vec::new() }
    }

    pub fn add_plant(&mut self, coordinate: Coordinate, plant_type: VegetationType) {
        self.plants.push((coordinate, plant_type));
    }

    pub fn update(&mut self) {
        for (_, plant) in self.plants.iter_mut() {
            plant.update();
        }
    }

    pub fn get_plant_at(&mut self, coordinate: &Coordinate) -> Option<&mut VegetationType> {
        self.plants
            .iter_mut()
            .find(|(pos, _)| pos == coordinate)
            .map(|(_, plant)| plant)
    }
}
