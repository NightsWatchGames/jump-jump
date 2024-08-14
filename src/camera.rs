use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::player::{FallState, JumpState, Player, INITIAL_PLAYER_POS};

pub const INITIAL_CAMERA_POS: Vec3 = Vec3::new(-5.0, 8.0, 5.0);

#[derive(Debug, Resource)]
pub struct CameraMoveState {
    step: Vec3,
    player_pos: Vec3,
}

impl Default for CameraMoveState {
    fn default() -> Self {
        Self {
            step: Vec3::ZERO,
            player_pos: INITIAL_PLAYER_POS,
        }
    }
}

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

    // see issue https://github.com/bevyengine/bevy/issues/7352
    let hdr = if cfg!(target_arch = "wasm32") {
        false
    } else {
        true
    };

    // camera
    let mut camera_command = commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(INITIAL_CAMERA_POS).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            hdr: hdr,
            ..default()
        },
        ..default()
    },));

    #[cfg(not(target_arch = "wasm32"))]
    {
        camera_command.insert(BloomSettings::default());
    }
}

pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 地面
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Plane3d::new(Vec3::Y, Vec2::new(1000000.0, 1000000.0))
                .mesh()
                .size(1000000.0, 1000000.0),
        ),
        material: materials.add(Color::srgb(0.95, 0.87, 0.88)),
        ..default()
    });
}

// 相机跟随玩家
pub fn move_camera(
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut camera_move_state: ResMut<CameraMoveState>,
    jump_state: Res<JumpState>,
    fall_state: Res<FallState>,
) {
    // 跳跃或摔落期间不移动相机
    if jump_state.completed && fall_state.completed {
        let player = q_player.single();
        let mut camera = q_camera.single_mut();
        let camera_destination = INITIAL_CAMERA_POS + player.translation;

        // 检测player是否移动，重新计算step
        if camera_move_state.player_pos.distance(player.translation) > 0.1 {
            let delta = camera_destination - camera.translation;
            camera_move_state.step = 0.05 * delta;
            camera_move_state.player_pos = player.translation;
        }

        if camera.translation.distance(camera_destination)
            > Vec3::ZERO.distance(camera_move_state.step)
        {
            camera.translation = camera.translation + camera_move_state.step;
        }
    }
}
