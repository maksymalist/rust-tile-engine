use bevy::{prelude::{Res, ResMut}, core::Time};

use super::{TileMap, PreviousTile, TileType};

pub struct Player {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    pub fn movement(&mut self, direction: Direction, tile_map: ResMut<TileMap>, _ti: Res<Time>, mut prev: ResMut<Vec<PreviousTile>>){
        let speed: usize = 1;
        prev.pop();
        prev.push(tile_map.get().to_vec()[self.y][self.y]._type);

        if self.y > 0 && self.x > 0 && self.y < tile_map.get().len() && self.x < tile_map.get()[0].len(){
            match direction {
                Direction::Up => self.wall_collision_check(tile_map, self.x, self.y - speed),
                Direction::Down => self.wall_collision_check(tile_map, self.x, self.y + speed),
                Direction::Left => self.wall_collision_check(tile_map, self.x - speed, self.y),
                Direction::Right => self.wall_collision_check(tile_map, self.x + speed, self.y)
            }
        }
    }

    pub fn get(&mut self) -> Player{
        Player { x: self.x, y: self.y }
    }

    fn wall_collision_check(&mut self, tile_map: ResMut<TileMap>, x: usize, y: usize) {
        if tile_map.get().to_vec()[y][x]._type != TileType::Wall{
            self.x = x;
            self.y = y;
        }
    }

}