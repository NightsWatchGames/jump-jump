use bevy::prelude::*;

// 当前所站的平台
#[derive(Debug, Component)]
pub struct CurrentPlatform;

// 下一个平台
#[derive(Debug, Component)]
pub struct NextPlatform;

pub fn setup_first_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        CurrentPlatform,
    ));
}

// TODO 生成下一个平台
pub fn generate_next_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_current_platform: Query<&Transform, With<CurrentPlatform>>,
    q_next_platform: Query<Entity, With<NextPlatform>>,
) {
    if q_next_platform.is_empty() {
        for current_platform in &q_current_platform {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(
                        current_platform.translation.x + 2.0,
                        0.5,
                        current_platform.translation.z,
                    ),
                    ..default()
                },
                NextPlatform,
            ));
        }
    }
}
