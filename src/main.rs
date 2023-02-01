use crate::camera::*;
use crate::platform::*;
use crate::player::*;
use bevy::prelude::*;

mod camera;
mod platform;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CameraMoveState::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        .add_startup_system(setup_first_platform)
        .add_startup_system(setup_player)
        .add_system(generate_next_platform)
        .add_system(move_camera)
        .add_system(player_jump)
        .run();
}