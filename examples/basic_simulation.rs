use the_nonhumankind::map::{Map, Coordinate};
use the_nonhumankind::entities::Individual;
use the_nonhumankind::rules::{IndividualRules, MapRules};
use the_nonhumankind::utils::{save_simulation_state, load_simulation_state};

fn main() {
    let mut map = Map::new(20, 20);
    let mut individuals = vec![
        Individual::new(1, Coordinate::new(5, 5), "5678123456".to_string()).unwrap(),
        Individual::new(2, Coordinate::new(15, 15), "6789234567".to_string()).unwrap(),
        Individual::new(3, Coordinate::new(10, 10), "7890345678".to_string()).unwrap(),
    ];
    let individual_rules = IndividualRules::new();
    let map_rules = MapRules;

    println!("Starting basic simulation with {} individuals on a {}x{} map", individuals.len(), map.width, map.height);

    for turn in 1..=100 {
        println!("\nTurn {}:", turn);

        map_rules.apply(&mut map);

        for individual in individuals.iter_mut() {
            let old_position = individual.position;
            individual_rules.apply(individual, &map);
            println!("  Individual {}: position: {:?} -> {:?}, Energy: {}, HP: {}",
                     individual.id, old_position, individual.position, individual.energy, individual.hp);
        }

        let avg_energy = individuals.iter().map(|i| i.energy).sum::<u32>() as f32 / individuals.len() as f32;
        let avg_hp = individuals.iter().map(|i| i.hp).sum::<u32>() as f32 / individuals.len() as f32;
        println!("Average energy: {:.2}, Average HP: {:.2}", avg_energy, avg_hp);

        if individuals.iter().all(|i| i.hp == 0) {
            println!("All individuals have perished. Ending simulation.");
            break;
        }
    }

    println!("\nSimulation ended.");

    // Example of saving and loading simulation state
    save_simulation_state(&individuals, "simulation_state.txt").unwrap();
    let loaded_individuals = load_simulation_state("simulation_state.txt").unwrap();
    println!("Loaded {} individuals from file", loaded_individuals.len());
}
