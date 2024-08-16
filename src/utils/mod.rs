use std::io::{self, Write, BufRead};
use std::fs::File;
use crate::entities::Individual;

pub fn save_simulation_state(individuals: &[Individual], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for individual in individuals.iter() {
        writeln!(file, "{}", individual.to_state_string())?;
    }
    Ok(())
}

pub fn load_simulation_state(filename: &str) -> io::Result<Vec<Individual>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut individuals = Vec::new();

    for line in reader.lines() {
        let state_string = line?;
        if let Ok(individual) = Individual::from_state_string(&state_string) {
            individuals.push(individual);
        }
    }

    Ok(individuals)
}
