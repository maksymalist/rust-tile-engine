pub mod lib;
use lib::{TileMap, Tile, TileType, PreviousTile, Player, Direction};

use bevy::core::FixedTimestep;
use bevy::prelude::*;
//use rand::prelude::*;

use std::time::Duration;
use std::thread::sleep;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const PADDING: f32 = 20.0;

const PLAYER_ORIGIN_X: usize = 2;
const PLAYER_ORIGIN_Y: usize = 2;

const CELL_SIZE: f32 = 20.0;


const TILE_COLOR_EMPTY: Color = Color::rgb(0.0, 1.0, 1.0);
const TILE_COLOR_GRASS: Color = Color::rgb(0.0, 1.0, 0.1);
const TILE_COLOR_WALL: Color = Color::rgb(0.0, 0.0, 0.0);
const TILE_COLOR_PLAYER: Color = Color::rgb(1.0, 1.0, 1.0);




fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


fn spawn_tiles(mut commands: Commands, tm: ResMut<TileMap>, mut player: ResMut<Player>) {

    let tile_map: Vec<Vec<Tile>> = tm.get().to_vec();


    //(WIDTH * -1.0) / 2.0) as f32, (HEIGHT / 2.0) as f32,

    for (x, i) in tile_map.iter().enumerate(){
       for (y, tile) in i.iter().enumerate() {

        let p: Player = player.get();
        let tile_type: TileType = if p.x == y && p.y == x { TileType::Player } else { tile._type };

        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: match tile_type {
                    TileType::Empty => TILE_COLOR_EMPTY,
                    TileType::Grass => TILE_COLOR_GRASS,
                    TileType::Wall => TILE_COLOR_WALL,
                    TileType::Player => TILE_COLOR_PLAYER
                },
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, 0.0),
                translation: Vec3::new(
                    (WIDTH * -1.0)/ 2.0 + (y as f32 * CELL_SIZE) + PADDING,
                    (HEIGHT / 2.0) - (x as f32 * CELL_SIZE) - PADDING,
                    0.0,
                ),
                ..default()
            },
            ..default()
        })
        .insert(Tile {
            _type: lib::TileType::Empty
        });
        
       }
    }
}

fn despawn_system<M: Component>(
    mut commands: Commands, 
    query: Query<Entity, With<M>>
) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>, 
    tm: ResMut<TileMap>,
    mut player: ResMut<Player>,
    time: Res<Time>,
    prev: ResMut<Vec<PreviousTile>>

) { 
    sleep(Duration::from_millis(20));
    if keyboard_input.pressed(KeyCode::Left) {
        player.movement(Direction::Left, tm, time, prev);
    } else if keyboard_input.pressed(KeyCode::Down) {
        player.movement(Direction::Down, tm, time, prev);
    } else if keyboard_input.pressed(KeyCode::Up) {
        player.movement(Direction::Up, tm, time, prev);
    } else if keyboard_input.pressed(KeyCode::Right) {
        player.movement(Direction::Right, tm, time, prev);
    } else if keyboard_input.pressed(KeyCode::Space) {
        println!("space");
    }
}

fn main() {

    let tile_map: TileMap = TileMap::new();
    let player: Player = Player::new(PLAYER_ORIGIN_X, PLAYER_ORIGIN_Y);
    let prev: Vec<PreviousTile> = vec![tile_map.get().to_vec()[PLAYER_ORIGIN_Y][PLAYER_ORIGIN_X]._type];


    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.04, 1.0)))
        .insert_resource(WindowDescriptor {
            title: "Cool Rpg".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .add_startup_system(setup_camera)
        .add_system(keyboard_input)
        .insert_resource(tile_map)
        .insert_resource(player)
        .insert_resource(prev)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01))
                .with_system(spawn_tiles)
                .with_system(despawn_system::<Tile>)
                
        )
        .add_plugins(DefaultPlugins)
        .run();
}
