use bevy::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Grass,
    Player
}

#[derive(Component, Clone, Debug, Copy, PartialEq)]
pub struct Tile {
    pub _type: TileType,
}

pub type PreviousTile = TileType;

impl Tile {
    pub fn new(_type: TileType) -> Self {
        Self {
            _type: _type,
        }
    }
}