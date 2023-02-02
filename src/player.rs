use bevy::prelude::{shape::CapsuleUvProfile, *};
use std::f32::consts::{PI, TAU};
use std::time::Instant;

use crate::platform::PlatformShape;
use crate::ui::GameOverEvent;
use crate::{
    platform::{CurrentPlatform, NextPlatform},
    ui::Score,
};

pub const INITIAL_PLAYER_POS: Vec3 = Vec3::new(0.0, 1.5, 0.0);

// 蓄力
#[derive(Debug, Resource)]
pub struct Accumulator(pub Option<Instant>);

// 跳跃状态
#[derive(Debug, Resource)]
pub struct JumpState {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    // 跳跃动画时长，秒
    pub animation_duration: f32,
    pub completed: bool,
}
impl Default for JumpState {
    fn default() -> Self {
        Self {
            start_pos: Vec3::ZERO,
            end_pos: Vec3::ZERO,
            animation_duration: 0.0,
            completed: true,
        }
    }
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
            transform: Transform::from_translation(INITIAL_PLAYER_POS),
            ..default()
        },
        Player,
    ));
}

pub fn player_jump(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut score: ResMut<Score>,
    mut accumulator: ResMut<Accumulator>,
    mut jump_state: ResMut<JumpState>,
    time: Res<Time>,
    mut game_over_ew: EventWriter<GameOverEvent>,
    q_player: Query<&Transform, With<Player>>,
    q_current_platform: Query<(Entity, &Transform, &PlatformShape), With<CurrentPlatform>>,
    q_next_platform: Query<
        (Entity, &Transform, &PlatformShape),
        (With<NextPlatform>, Without<Player>),
    >,
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
        let (current_platform_entity, current_platform, current_platform_shape) =
            q_current_platform.single();
        let (next_platform_entity, next_platform, next_platform_shape) = q_next_platform.single();
        let player = q_player.single();

        // 计算跳跃后的落点位置
        let landing_pos = if (next_platform.translation.x - current_platform.translation.x) < 0.1 {
            Vec3::new(
                player.translation.x,
                INITIAL_PLAYER_POS.y,
                player.translation.z
                    - 3.0 * accumulator.0.as_ref().unwrap().elapsed().as_secs_f32(),
            )
        } else {
            Vec3::new(
                player.translation.x
                    + 3.0 * accumulator.0.as_ref().unwrap().elapsed().as_secs_f32(),
                INITIAL_PLAYER_POS.y,
                player.translation.z,
            )
        };
        dbg!(accumulator.0.as_ref().unwrap().elapsed().as_secs_f32());

        // 蓄力极短，跳跃后仍在当前平台上
        // 蓄力正常，跳跃到下一平台
        if current_platform_shape.is_landed_on_platform(current_platform.translation, landing_pos)
            || next_platform_shape.is_landed_on_platform(next_platform.translation, landing_pos)
        {
            jump_state.start_pos = player.translation;
            jump_state.end_pos = landing_pos;
            // 跳跃动画时长随距离而变化
            jump_state.animation_duration =
                (accumulator.0.as_ref().unwrap().elapsed().as_secs_f32() / 2.0).max(0.5);
            jump_state.completed = false;

            if next_platform_shape.is_landed_on_platform(next_platform.translation, landing_pos) {
                // 分数加1
                score.0 += 1;
                commands
                    .entity(next_platform_entity)
                    .remove::<NextPlatform>();
                commands
                    .entity(next_platform_entity)
                    .insert(CurrentPlatform);
                commands
                    .entity(current_platform_entity)
                    .remove::<CurrentPlatform>();
            }

        // 蓄力不足或蓄力过度，游戏结束
        } else {
            info!("Game over!");
            game_over_ew.send_default();
        }

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

        // TODO 围绕中心点圆周?运动
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
            -(1.0 / jump_state.animation_duration) * PI * time.delta_seconds(),
        );

        let mut clone_player = player.clone();
        clone_player.translate_around(around_point, quat);
        if clone_player.translation.y < INITIAL_PLAYER_POS.y {
            player.translation = jump_state.end_pos;
            player.rotation = Quat::IDENTITY;

            // 结束跳跃
            jump_state.completed = true;
        } else {
            player.translate_around(around_point, quat);

            // 自身旋转
            player.rotate_local_axis(
                rotate_axis,
                -(1.0 / jump_state.animation_duration) * TAU * time.delta_seconds(),
            );
        }
    }
}

// 角色蓄力效果
pub fn animate_player_accumulation(
    accumulator: Res<Accumulator>,
    mut q_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = q_player.single_mut();
    match accumulator.0 {
        Some(_) => {
            player.scale.x = (player.scale.x + 0.0006 * time.elapsed_seconds()).min(1.3);
            player.scale.y = (player.scale.y - 0.0008 * time.elapsed_seconds()).max(0.6);
            player.scale.z = (player.scale.z + 0.0006 * time.elapsed_seconds()).min(1.3);
        }
        None => {
            player.scale = Vec3::ONE;
        }
    }
}
