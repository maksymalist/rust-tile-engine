use crate::lib::{Tile, TileType, custom_map};

// Structs and Implementations

#[derive(Debug)]
pub struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new() -> Self {
        let tiles = custom_map();
        Self {
            tiles,
        }
    }

    pub fn get(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn set_tile(&mut self, x: usize, y: usize, _type: TileType) {
        self.tiles[y][x]._type = _type;
    }

}
