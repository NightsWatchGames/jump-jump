use bevy::prelude::{shape::CapsuleUvProfile, *};
use std::f32::consts::{PI, TAU};
use std::time::Instant;

use crate::{
    platform::{CurrentPlatform, NextPlatform},
    ui::Score,
};

pub const PLAYER_INITIAL_POS: Vec3 = Vec3::new(0.0, 1.5, 0.0);
// 跳跃动画时长，秒
pub const JUMP_ANIMATION_DURATION: f32 = 1.0;

// 蓄力
#[derive(Debug, Resource)]
pub struct Accumulator(pub Option<Instant>);

// 跳跃状态
#[derive(Debug, Resource)]
pub struct JumpState {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub completed: bool,
}

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
    mut score: ResMut<Score>,
    mut accumulator: ResMut<Accumulator>,
    mut jump_state: ResMut<JumpState>,
    time: Res<Time>,
    q_player: Query<&Transform, With<Player>>,
    q_current_platform: Query<Entity, With<CurrentPlatform>>,
    q_next_platform: Query<(Entity, &Transform), (With<NextPlatform>, Without<Player>)>,
) {
    // 如果上一跳未完成则忽略
    if buttons.just_pressed(MouseButton::Left) && jump_state.completed {
        // 开始蓄力
        accumulator.0 = time.last_update();
    }
    if buttons.just_released(MouseButton::Left) && jump_state.completed {
        if q_next_platform.is_empty() {
            warn!("There is no next platform");
            return;
        }
        // TODO 计算跳跃后的落点位置
        // TODO 蓄力极短，跳跃后仍在当前平台上
        // TODO 蓄力不足或蓄力过度，游戏结束
        let (next_platform_entity, next_platform) = q_next_platform.single();
        let player = q_player.single();

        jump_state.start_pos = player.translation;
        jump_state.end_pos = Vec3::new(
            next_platform.translation.x,
            PLAYER_INITIAL_POS.y,
            next_platform.translation.z,
        );
        jump_state.completed = false;

        // 分数加1
        score.0 += 1;

        commands
            .entity(next_platform_entity)
            .remove::<NextPlatform>();
        commands
            .entity(next_platform_entity)
            .insert(CurrentPlatform);
        commands
            .entity(q_current_platform.single())
            .remove::<CurrentPlatform>();

        // 结束蓄力
        accumulator.0 = None;
    }
}

pub fn animate_jump(
    mut jump_state: ResMut<JumpState>,
    time: Res<Time>,
    mut q_player: Query<&mut Transform, With<Player>>,
) {
    if !jump_state.completed {
        let mut player = q_player.single_mut();

        // 围绕中心点圆周运动
        let around_point = Vec3::new(
            (jump_state.start_pos.x + jump_state.end_pos.x) / 2.0,
            (jump_state.start_pos.y + jump_state.end_pos.y) / 2.0,
            (jump_state.start_pos.z + jump_state.end_pos.z) / 2.0,
        );

        let rotate_axis = if (jump_state.end_pos.x - jump_state.start_pos.x) < 0.1 {
            Vec3::X
        } else {
            Vec3::Z
        };
        let quat = Quat::from_axis_angle(
            rotate_axis,
            -(1.0 / JUMP_ANIMATION_DURATION) * PI * time.delta_seconds(),
        );

        let mut clone_player = player.clone();
        clone_player.translate_around(around_point, quat);
        if clone_player.translation.y < PLAYER_INITIAL_POS.y {
            player.translation = jump_state.end_pos;
            player.rotation = Quat::IDENTITY;

            // 结束跳跃
            jump_state.completed = true;
        } else {
            player.translate_around(around_point, quat);

            // 自身旋转
            player.rotate_local_axis(
                rotate_axis,
                -(1.0 / JUMP_ANIMATION_DURATION) * TAU * time.delta_seconds(),
            );
        }
    }
}
