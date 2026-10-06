mod maze_algo;
mod modules;
use bevy::prelude::*;
use maze_algo::Maze;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
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
