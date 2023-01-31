use bevy::prelude::*;

use crate::{platform::CurrentPlatform, player::Player};

const INITIAL_CAMERA_POS: Vec3 = Vec3::new(-5.0, 8.0, 5.0);

pub fn setup_camera(mut commands: Commands) {
    // 方向光
    // TODO 阴影
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 10.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(INITIAL_CAMERA_POS).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 地面
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000000.0 })),
        material: materials.add(Color::rgb(0.95, 0.87, 0.88).into()),
        ..default()
    });
}

// TODO 相机跟随玩家
pub fn move_camera(
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = q_player.single();
    let mut camera = q_camera.single_mut();
    camera.translation = INITIAL_CAMERA_POS + player.translation;
}
