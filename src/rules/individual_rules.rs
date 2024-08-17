use crate::entities::Individual;
use crate::map::{Map, Coordinate};
use super::MovementRules;
use rand::Rng;

pub struct IndividualRules {
    movement_rules: MovementRules,
}

impl IndividualRules {
    pub fn new() -> Self {
        IndividualRules {
            movement_rules: MovementRules,
        }
    }

    pub fn apply(&self, individual: &mut Individual, map: &mut Map) {
        individual.update();

        if individual.energy < individual.genetics.energy_capacity / 2 {
            self.search_for_food(individual, map);
        } else {
            self.random_move(individual, map);
        }

        individual.eat(map);
    }

    fn search_for_food(&self, individual: &mut Individual, map: &Map) {
        let directions = [
            Coordinate::new(0, 1),
            Coordinate::new(1, 0),
            Coordinate::new(0, usize::MAX),
            Coordinate::new(usize::MAX, 0),
        ];

        for &direction in &directions {
            let new_position = individual.position + direction;
            if map.is_valid_position(&new_position) && map.vegetation.get_plant_at(&new_position).is_some() {
                self.movement_rules.move_individual(individual, new_position, map);
                return;
            }
        }

        // If no food found, move randomly
        self.random_move(individual, map);
    }

    fn random_move(&self, individual: &mut Individual, map: &Map) {
        let directions = [
            Coordinate::new(0, 1),
            Coordinate::new(1, 0),
            Coordinate::new(0, usize::MAX),
            Coordinate::new(usize::MAX, 0),
        ];

        let mut rng = rand::thread_rng();
        let new_position = individual.position + directions[rng.gen_range(0..4)];
        self.movement_rules.move_individual(individual, new_position, map);
    }
}
