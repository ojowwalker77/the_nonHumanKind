use crate::entities::Individual;
use crate::map::{Map, Coordinate};
use super::MovementRules;

pub struct IndividualRules {
    movement_rules: MovementRules,
}

impl IndividualRules {
    pub fn new() -> Self {
        IndividualRules {
            movement_rules: MovementRules,
        }
    }

    pub fn apply(&self, individual: &mut Individual, map: &Map) {
        individual.update();

        if individual.needs_food() {
            self.search_for_food(individual, map);
        }
    }

    fn search_for_food(&self, individual: &mut Individual, map: &Map) {
        let directions = [
            Coordinate::new(0, 1),
            Coordinate::new(1, 0),
            Coordinate::new(0, usize::MAX), // Moving up (y-1)
            Coordinate::new(usize::MAX, 0), // Moving left (x-1)
        ];

        let direction = directions[fastrand::usize(..4)];
        let new_position = individual.position + direction;

        if self.movement_rules.can_move(&new_position, map) {
            individual.move_to(new_position);
        }
    }
}
