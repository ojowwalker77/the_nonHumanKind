use crate::map::Map;

pub struct MapRules;

impl MapRules {
    pub fn apply(&self, map: &mut Map) {
        // For now, we'll just update the map's state
        // In the future, this could include things like resource regeneration,
        // natural disasters, etc.
        map.update();
    }
}
