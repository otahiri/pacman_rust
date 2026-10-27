mod maze_algo;
mod modules;
use bevy::prelude::*;
use maze_algo::Maze;
#[derive(Component)]
struct Wall;

fn main() {
    let maze = Maze::new(16, 16);
    App::new().add_plugins(DefaultPlugins).insert_resource(maze).add_systems(Startup, setup).add_systems(Update, make_walls).run();

}

fn setup(mut command: Commands) {
    command.spawn(Camera2d::default());
}

fn make_walls(mut  command: Commands, asset_serve: Res<AssetServer>, keyboard: Res<ButtonInput<KeyCode>>, wall:Option<Single<Entity, With<Wall>>>) {

    if keyboard.just_pressed(KeyCode::Space) {
        if wall.is_none() {
            command.spawn((Sprite::from_image(asset_serve.load("heart.png")), Wall));
        }
    }
    else if keyboard.just_pressed(KeyCode::Backspace) {
        if let Some(p) = wall {
            println!("hello");
             command.entity(*p).despawn();
        }
    }
}
