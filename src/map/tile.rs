#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerrainType {
    Plains,
    Forest,
    Mountain,
    Water,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub terrain: TerrainType,
}

impl Default for Tile {
    fn default() -> Self {
        Tile { terrain: TerrainType::Plains }
    }
}
