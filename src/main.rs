mod map;
mod entities;
mod rules;
mod vegetation;
mod plugins;
mod logger;
mod utils;

use map::Map;
use entities::Individual;
use rules::{IndividualRules, MapRules};

fn main() {
    let mut map = Map::new(1000, 1000);
    let mut individual = Individual::new(1, map::Coordinate::new(500, 500));
    let individual_rules = IndividualRules::new();
    let map_rules = MapRules;

    for i in 0..100 {
        map_rules.apply(&mut map);
        individual_rules.apply(&mut individual, &map);
        println!("Turn {}: Individual id: {}, position: {:?}, Energy: {}, HP: {}",
                 i, individual.id, individual.position, individual.energy, individual.hp);
    }
}
