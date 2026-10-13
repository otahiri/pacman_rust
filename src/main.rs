mod maze_algo;
mod modules;
use bevy::prelude::*;
use maze_algo::Maze;

fn main() {
    let mut maze = Maze::new(15, 15);
    maze.create_maze();
    let (width, height) = maze.get_size();
    for y in 0..height {
        for x in 0..width {
            print!("{}", maze.get_tile(x, y));
        }
        println!("")
    }
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn((
            Sprite {
                    image: asset_server.load("player/alive/0.png"),
                    ..default()
                },
                Transform::from_xyz(100.0, 10.0, 1.0),

            ));
}
