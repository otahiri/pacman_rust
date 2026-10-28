mod maze_algo;
mod modules;
use std::collections::HashSet;

use bevy::platform::collections::hash_set;
use bevy::prelude::*;
use maze_algo::Maze;

use crate::modules::Direction;

fn main() {
    let mut maze = Maze::new(10, 10);
    maze.create_maze();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(maze)
        .add_systems(Startup, setup)
        .add_systems(Startup, make_walls)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn(Camera2d::default());
}

fn make_walls(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
    maze: Res<Maze>,
) {
    let size: usize = 32;
    let vert: Handle<Image> = asset_server.load("walls/horizontanl_wall.png");
    let hori: Handle<Image> = asset_server.load("walls/vertical_wall.png");
    let mut walls_pos: HashSet<(usize, usize, bool)> = HashSet::new();
    let mut corners_pos: HashSet<(usize, usize, usize)> = HashSet::new();
    let (width, height) = maze.get_size();
    for y in 0..height {
        for x in 0..width {
            for dir in Direction::ALL {
                if maze.check_wall(x, y, &dir) {
                    walls_pos.insert((
                        (x * size) * 2 + 1,
                        (y * size) * 2 + 1,
                        matches!(&dir, Direction::South | Direction::North),
                    ));
                }
            }
        }
    }
    let corner_x: f32 = ((size * width) / 2 ) as f32;
    let corner_y: f32 = ((size * height) / 2 ) as f32;
    for (x, y, is_hor) in walls_pos {
        let wall = if is_hor { &hori } else { &vert };
        command.spawn((
            Sprite::from_image(wall.clone()),
            Transform::from_xyz(x as f32 - corner_x , y as f32 - corner_y , 0.0),
        ));
    }
}
