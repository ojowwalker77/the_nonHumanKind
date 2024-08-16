mod tile;
mod coordinate;

pub use self::tile::{Tile, TerrainType};
pub use self::coordinate::Coordinate;
use std::vec;



pub struct Map {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Map {



    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::with_capacity(height);
        for _ in 0..height {
            tiles.push(vec![Tile::default(); width]);
        }
        Map { width, height, tiles }
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

    pub fn update(&mut self) {
        // For now, this method is empty
        // In the future, it could update the state of each tile
    }
}
