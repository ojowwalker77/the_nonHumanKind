use crate::map::{Map, Coordinate};
use crate::entities::Individual;

pub struct MovementRules;

impl MovementRules {
    pub fn can_move(&self, to: &Coordinate, map: &Map) -> bool {
        // Check if the destination is within map bounds
        if let Some(tile) = map.get_tile(to) {
            // For now, allow movement to any tile except water
            !matches!(tile.terrain, crate::map::TerrainType::Water)
        } else {
            false
        }
    }

    pub fn move_individual(&self, individual: &mut Individual, to: Coordinate, map: &Map) {
        if self.can_move(&to, map) {
            individual.move_to(to);
        }
    }
}
