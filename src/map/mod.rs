mod tile;
mod coordinate;

pub use self::tile::{Tile, TerrainType};
pub use self::coordinate::Coordinate;

use crate::vegetation::VegetationSystem;

pub struct Map {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<Tile>>,
    pub vegetation: VegetationSystem,
}

impl Map {

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }


    pub fn is_valid_position(&self, coord: &Coordinate) -> bool {
        coord.x < self.width && coord.y < self.height
    }


    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::default(); width]; height];
        let vegetation = VegetationSystem::new();
        Map { width, height, tiles, vegetation }
    }

    pub fn get_tile(&self, coord: &Coordinate) -> Option<&Tile> {
        if coord.x < self.width && coord.y < self.height {
            Some(&self.tiles[coord.y][coord.x])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, coord: &Coordinate) -> Option<&mut Tile> {
        if coord.x < self.width && coord.y < self.height {
            Some(&mut self.tiles[coord.y][coord.x])
        } else {
            None
        }
    }

    pub fn eat_plant_at(&mut self, coordinate: &Coordinate) -> Option<u32> {
        self.vegetation.eat_plant_at(coordinate)
    }

    pub fn update(&mut self) {
        self.vegetation.update();
    }
}
