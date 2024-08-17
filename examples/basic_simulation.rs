use the_nonhumankind::map::{Coordinate, Map};
use the_nonhumankind::entities::Individual;
use the_nonhumankind::rules::{IndividualRules, MapRules};
use the_nonhumankind::statistics::SimulationStats;
use the_nonhumankind::vegetation::{VegetationType, Grass, FruitTree};
use the_nonhumankind::visualization::Visualizer;
use the_nonhumankind::utils::{save_simulation_state, load_simulation_state};
use std::{thread, time::Duration};
use rand::Rng;

fn main() {
    let width = 100;
    let height = 50;
    let mut map = Map::new(width, height);
    let visualizer = Visualizer::new(width, height);

    let mut rng = rand::thread_rng();

    // Add vegetation
    for x in 0..width {
        for y in 0..height {
            let random_number = rng.gen_range(0..100);
            if random_number < 5 {
                map.vegetation.add_plant(Coordinate::new(x, y), VegetationType::FruitTree(FruitTree::new()));
            } else if random_number < 50 {
                map.vegetation.add_plant(Coordinate::new(x, y), VegetationType::Grass(Grass::new()));
            }
        }
    }

    // Create more individuals
    let mut individuals = Vec::new();
    for i in 0..50 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        if let Ok(individual) = Individual::new(i, Coordinate::new(x, y), None) {
            individuals.push(individual);
        }
    }

    let individual_rules = IndividualRules::new();
    let map_rules = MapRules;
    let mut stats = SimulationStats::new();

    println!("Starting simulation with {} individuals on a {}x{} map", individuals.len(), map.width, map.height);

    let mut log_file = std::fs::File::create("simulation_log.txt").expect("Failed to create log file");

    for turn in 1..=1000 {
        map_rules.apply(&mut map);

        for individual in individuals.iter_mut() {
            individual_rules.apply(individual, &mut map);
        }

        stats.update(&individuals, &map);

        if turn % 10 == 0 {
            visualizer.render(&map, &individuals);
            println!("\nTurn: {}", turn);
            println!("Population: {}", individuals.len());
            println!("Average Energy: {:.2}", individuals.iter().map(|i| i.energy).sum::<u32>() as f32 / individuals.len() as f32);
            println!("Average HP: {:.2}", individuals.iter().map(|i| i.hp).sum::<u32>() as f32 / individuals.len() as f32);

            // Log detailed statistics
            use std::io::Write;
            writeln!(log_file, "Turn: {}", turn).unwrap();
            writeln!(log_file, "Population: {}", individuals.len()).unwrap();
            writeln!(log_file, "Average Energy: {:.2}", individuals.iter().map(|i| i.energy).sum::<u32>() as f32 / individuals.len() as f32).unwrap();
            writeln!(log_file, "Average HP: {:.2}", individuals.iter().map(|i| i.hp).sum::<u32>() as f32 / individuals.len() as f32).unwrap();
            writeln!(log_file, "Total Food Consumed: {}", individuals.iter().map(|i| i.stats.food_consumed).sum::<u32>()).unwrap();
            writeln!(log_file, "Total Distance Moved: {}", individuals.iter().map(|i| i.stats.total_distance).sum::<u32>()).unwrap();
            writeln!(log_file, "Vegetation Count: {}", map.vegetation.plant_count()).unwrap();
            writeln!(log_file, "").unwrap();
        }

        thread::sleep(Duration::from_millis(50));

        if individuals.is_empty() {
            println!("All individuals have perished. Ending simulation.");
            break;
        }

        // Add reproduction logic
        let mut new_individuals = Vec::new();
        for individual in &individuals {
            if individual.energy > individual.genetics.energy_capacity * 3 / 4 && rng.gen::<f32>() < (individual.genetics.reproduction_rate as f32 / 100.0) {
                if let Ok(mut child) = Individual::new(
                    individuals.len() as u64 + new_individuals.len() as u64 + 1,
                    individual.position,
                    Some(individual.dna.clone()),
                ) {
                    child.mutate(&mut rng);
                    new_individuals.push(child);
                }

            }
        }
        individuals.extend(new_individuals);

        // Remove dead individuals
        individuals.retain(|i| i.hp > 0);
    }

    println!("\nSimulation ended.");
    println!("\nFinal Statistics Report:");
    println!("{}", stats.generate_report());

    save_simulation_state(&individuals, "simulation_state.txt").unwrap();
    let loaded_individuals = load_simulation_state("simulation_state.txt").unwrap();
    println!("Loaded {} individuals from file", loaded_individuals.len());
}
