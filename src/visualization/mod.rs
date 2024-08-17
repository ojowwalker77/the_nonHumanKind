use termion::{color, style, clear, cursor};
use crate::map::{Map, Coordinate};
use crate::entities::Individual;
use crate::vegetation::VegetationType;

pub struct Visualizer {
    width: usize,
    height: usize,
}

impl Visualizer {
    pub fn new(width: usize, height: usize) -> Self {
        Visualizer { width, height }
    }

    pub fn render(&self, map: &Map, individuals: &[Individual]) {
        print!("{}", clear::All);
        print!("{}", cursor::Goto(1, 1));

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinate::new(x, y);
                if individuals.iter().any(|i| i.position == coord) {
                    print!("{}{}{}", color::Fg(color::Red), "I", style::Reset);
                } else if let Some(plant) = map.vegetation.get_plant_at(&coord) {
                    match plant {
                        VegetationType::Grass(_) => print!("{}{}{}", color::Fg(color::Green), ".", style::Reset),
                        VegetationType::FruitTree(_) => print!("{}{}{}", color::Fg(color::Yellow), "T", style::Reset),
                    }
                } else {
                    print!(" "); // Empty space
                }
            }
            println!();
        }
    }
}
