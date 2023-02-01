use bevy::prelude::{shape::CapsuleUvProfile, *};

use crate::platform::{CurrentPlatform, NextPlatform};

pub const PLAYER_INITIAL_POS: Vec3 = Vec3::new(0.0, 1.5, 0.0);

#[derive(Debug, Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            // TODO 换成圆柱体
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.2,
                rings: 0,
                depth: 0.5,
                latitudes: 16,
                longitudes: 32,
                uv_profile: CapsuleUvProfile::Aspect,
            })),
            material: materials.add(Color::PINK.into()),
            transform: Transform::from_translation(PLAYER_INITIAL_POS),
            ..default()
        },
        Player,
    ));
}

// TODO
pub fn player_jump(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut q_player: Query<&mut Transform, With<Player>>,
    q_current_platform: Query<Entity, With<CurrentPlatform>>,
    q_next_platform: Query<(Entity, &Transform), (With<NextPlatform>, Without<Player>)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if q_next_platform.is_empty() {
            warn!("There is no next platform");
            return;
        }
        let (next_platform_entity, next_platform) = q_next_platform.single();
        let mut player = q_player.single_mut();
        player.translation.x = next_platform.translation.x;
        player.translation.z = next_platform.translation.z;

        commands
            .entity(next_platform_entity)
            .remove::<NextPlatform>();
        commands
            .entity(next_platform_entity)
            .insert(CurrentPlatform);
        commands
            .entity(q_current_platform.single())
            .remove::<CurrentPlatform>();
    }
}
