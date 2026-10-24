mod maze_algo;
mod modules;
use bevy::prelude::*;
use maze_algo::Maze;
use hex_utilities::convert_decimal_to_hexadecimal;

fn main() {
    let mut maze = Maze::new(15, 15);
    maze.create_maze();
    let (width, height) = maze.get_size();
    for y in 0..height {
        for x in 0..width {
            let hex_num = convert_decimal_to_hexadecimal(maze.get_tile(x, y) as u64, false, None);
            print!("{}", hex_num);
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
