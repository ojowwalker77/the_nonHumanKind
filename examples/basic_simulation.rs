use the_nonhumankind::map::{Coordinate, Map};
use the_nonhumankind::entities::Individual;
use the_nonhumankind::rules::{IndividualRules, MapRules};
use the_nonhumankind::statistics::SimulationStats;
use the_nonhumankind::vegetation::{VegetationType, Grass, FruitTree};
use the_nonhumankind::utils::{save_simulation_state, load_simulation_state};

fn main() {
    let mut map = Map::new(20, 20);

    for x in 0..20 {
        for y in 0..20 {
            if (x + y) % 5 == 0 {
                map.vegetation.add_plant(Coordinate::new(x, y), VegetationType::FruitTree(FruitTree::new()));
            } else if (x + y) % 2 == 0 {
                map.vegetation.add_plant(Coordinate::new(x, y), VegetationType::Grass(Grass::new()));
            }
        }
    }

    let mut individuals = vec![
        Individual::new(1, Coordinate::new(5, 5), "5678123456".to_string()).unwrap(),
        Individual::new(2, Coordinate::new(15, 15), "6789234567".to_string()).unwrap(),
        Individual::new(3, Coordinate::new(10, 10), "7890345678".to_string()).unwrap(),
    ];
    let individual_rules = IndividualRules::new();
    let map_rules = MapRules;
    let mut stats = SimulationStats::new();

    println!("Starting basic simulation with {} individuals on a {}x{} map", individuals.len(), map.width, map.height);

    for turn in 1..=100 {
        println!("\nTurn {}:", turn);

        map_rules.apply(&mut map);

        for individual in individuals.iter_mut() {
            let old_position = individual.position;
            individual_rules.apply(individual, &mut map);
            println!("  Individual {}: position: {:?} -> {:?}, Energy: {}, HP: {}",
                     individual.id, old_position, individual.position, individual.energy, individual.hp);
        }

        stats.update(&individuals, &map);

        if turn % 10 == 0 || individuals.iter().all(|i| i.hp == 0) {
            println!("\nStatistics Report:");
            println!("{}", stats.generate_report());
        }

        if individuals.iter().all(|i| i.hp == 0) {
            println!("All individuals have perished. Ending simulation.");
            break;
        }
    }

    println!("\nSimulation ended.");
    println!("\nFinal Statistics Report:");
    println!("{}", stats.generate_report());

    save_simulation_state(&individuals, "simulation_state.txt").unwrap();
    let loaded_individuals = load_simulation_state("simulation_state.txt").unwrap();
    println!("Loaded {} individuals from file", loaded_individuals.len());
}
