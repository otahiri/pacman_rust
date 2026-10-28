mod maze_algo;
mod modules;
use bevy::prelude::*;
use maze_algo::Maze;

use crate::modules::Direction;

fn main() {
    let maze = Maze::new(16, 16);
    App::new().add_plugins(DefaultPlugins).insert_resource(maze).add_systems(Startup, setup).add_systems(Update, make_walls).run();

}

fn setup(mut command: Commands) {
    command.spawn(Camera2d::default());
}

fn make_walls(mut  command: Commands, asset_server: Res<AssetServer>, keyboard: Res<ButtonInput<KeyCode>>, maze: Res<Maze>) {

    let vert: Handle<Image> = asset_server.load("walls/horizontanl_wall.png");
    let hori: Handle<Image> = asset_server.load("walls/vertical_wall.png");
    let (width, height) = maze.get_size();
    let size: usize = 32;
    for y in 0..height {
        for x in 0..width {
            let screen_x: usize = x * size;
            let screen_y: usize = y * size;
            if y % 2 == 0 {
                if x % 2 == 1 {
                    let north: Direction = Direction::North;
                    if (1 << north.shift() == 1) {
                        command.spawn((Sprite::from_image(hori.clone()), Transform::from_xyz(screen_x as f32, screen_y as f32, 0.0)));
                    }
                }
            }
            else {

                    let west: Direction = Direction::West;
                    if (1 << west.shift() &  == 1) {
                        command.spawn((Sprite::from_image(hori.clone()), Transform::from_xyz(screen_x as f32, screen_y as f32, 0.0)));
                    }
            }

            todo!("make algo ")
        }

    }
    if keyboard.just_pressed(KeyCode::Space) {
        if wall.is_none() {
            command.spawn((Sprite::from_image(asset_server.load("heart.png")), Wall));
        }
    }
    else if keyboard.just_pressed(KeyCode::Backspace) {
        if let Some(p) = wall {
            println!("hello");
             command.entity(*p).despawn();
        }
    }
}
