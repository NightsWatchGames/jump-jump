use bevy::audio::AudioSink;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_hanabi::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use crate::platform::PlatformShape;
use crate::ui::{GameSounds, GameState, ScoreUpEvent, ScoreUpQueue};
use crate::{
    platform::{CurrentPlatform, NextPlatform},
    ui::Score,
};

pub const INITIAL_PLAYER_POS: Vec3 = Vec3::new(0.0, 1.5, 0.0);

// 蓄力
#[derive(Debug, Resource)]
pub struct Accumulator(pub Option<Instant>);

#[derive(Debug, Component)]
pub struct AccumulationSound;

#[derive(Debug, Resource)]
pub struct PrepareJumpTimer(pub Timer);

// 跳跃状态
#[derive(Debug, Resource)]
pub struct JumpState {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    // 跳跃动画时长，秒
    pub animation_duration: f32,
    pub falled: bool,
    pub completed: bool,
}
impl Default for JumpState {
    fn default() -> Self {
        Self {
            start_pos: Vec3::ZERO,
            end_pos: Vec3::ZERO,
            animation_duration: 0.0,
            falled: false,
            completed: true,
        }
    }
}
impl JumpState {
    pub fn animate_jump(&mut self, start_pos: Vec3, end_pos: Vec3, animation_duration: f32) {
        info!("Start jump!");
        self.start_pos = start_pos;
        self.end_pos = end_pos;
        self.animation_duration = animation_duration;
        self.completed = false;
    }
}

// 摔落状态
#[derive(Debug, Resource)]
pub struct FallState {
    pub pos: Vec3,
    pub fall_type: FallType,
    // 是否完成倾斜动作
    pub tilt_completed: bool,
    // 是否所有动作完成
    pub completed: bool,
    pub played_sound: bool,
}
#[derive(Debug)]
pub enum FallType {
    // 笔直下落
    Straight,
    // 先倾斜再下落，Vec3代表倾斜方向
    Tilt(Vec3),
}
impl Default for FallState {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            fall_type: FallType::Straight,
            tilt_completed: true,
            completed: true,
            played_sound: true,
        }
    }
}
impl FallState {
    pub fn animate_straight_fall(&mut self, pos: Vec3) {
        info!("Start straight fall!");
        self.pos = pos;
        self.fall_type = FallType::Straight;
        self.completed = false;
        self.played_sound = false;
    }
    pub fn animate_tilt_fall(&mut self, pos: Vec3, direction: Vec3) {
        info!("Start tilt fall!");
        self.pos = pos;
        self.fall_type = FallType::Tilt(direction);
        self.tilt_completed = false;
        self.completed = false;
        self.played_sound = false;
    }
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct GenerateAccumulationParticleEffectTimer(pub Timer);

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_sounds: Res<GameSounds>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.2, 0.5).mesh()),
            material: materials.add(Color::PINK),
            transform: Transform::from_translation(INITIAL_PLAYER_POS),
            ..default()
        },
        Player,
    ));
    commands.spawn(AudioBundle {
        source: game_sounds.start.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

pub fn player_jump(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut score: ResMut<Score>,
    mut accumulator: ResMut<Accumulator>,
    mut jump_state: ResMut<JumpState>,
    mut fall_state: ResMut<FallState>,
    mut score_up_queue: ResMut<ScoreUpQueue>,
    prepare_jump_timer: Res<PrepareJumpTimer>,
    time: Res<Time<Real>>,
    game_sounds: Res<GameSounds>,
    q_accumulation_sound: Query<&AudioSink, With<AccumulationSound>>,
    q_player: Query<&Transform, With<Player>>,
    q_current_platform: Query<(Entity, &Transform, &PlatformShape), With<CurrentPlatform>>,
    q_next_platform: Query<
        (Entity, &Transform, &PlatformShape),
        (With<NextPlatform>, Without<Player>),
    >,
) {
    if !prepare_jump_timer.0.finished() {
        // 防止从主菜单点击进入Playing状态时立即跳一次
        return;
    }
    // 如果上一跳未完成则忽略
    if buttons.just_pressed(MouseButton::Left) && jump_state.completed && fall_state.completed {
        // 开始蓄力
        accumulator.0 = time.last_update();
        commands.spawn((
            AccumulationSound,
            AudioBundle {
                source: game_sounds.accumulation.clone(),
                settings: PlaybackSettings::LOOP,
            },
        ));
    }
    if buttons.just_released(MouseButton::Left)
        && jump_state.completed
        && fall_state.completed
        && accumulator.0.is_some()
    {
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
        dbg!(player.translation);
        dbg!(accumulator.0.as_ref().unwrap().elapsed().as_secs_f32());

        // 跳跃动画时长随距离而变化
        jump_state.animate_jump(
            player.translation,
            landing_pos,
            (accumulator.0.as_ref().unwrap().elapsed().as_secs_f32() / 2.0).max(0.5),
        );

        // 蓄力极短，跳跃后仍在当前平台上
        // 蓄力正常，跳跃到下一平台
        if current_platform_shape.is_landed_on_platform(current_platform.translation, landing_pos)
            || next_platform_shape.is_landed_on_platform(next_platform.translation, landing_pos)
        {
            jump_state.falled = false;
            if next_platform_shape.is_landed_on_platform(next_platform.translation, landing_pos) {
                // 分数加1
                score.0 += 1;
                score_up_queue.0.push(ScoreUpEvent {
                    pos: Vec3::new(landing_pos.x, landing_pos.y + 0.5, landing_pos.z),
                });

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

        // 蓄力不足或蓄力过度，角色摔落
        } else {
            jump_state.falled = true;
            if current_platform_shape.is_touched_player(
                current_platform.translation,
                landing_pos,
                0.2,
            ) {
                info!("Player touched current platform");
                let fall_direction = if landing_pos.x == player.translation.x {
                    Vec3::NEG_X
                } else {
                    Vec3::NEG_Z
                };
                fall_state.animate_tilt_fall(landing_pos, fall_direction);
            } else if next_platform_shape.is_touched_player(
                next_platform.translation,
                landing_pos,
                0.2,
            ) {
                info!("Player touched next platform");
                let fall_direction = if landing_pos.x == player.translation.x {
                    if landing_pos.z < next_platform.translation.z {
                        Vec3::NEG_X
                    } else {
                        Vec3::X
                    }
                } else {
                    if landing_pos.x < next_platform.translation.x {
                        Vec3::Z
                    } else {
                        Vec3::NEG_Z
                    }
                };
                fall_state.animate_tilt_fall(landing_pos, fall_direction);
            } else {
                fall_state.animate_straight_fall(landing_pos);
            }
        }

        // 结束蓄力
        accumulator.0 = None;
        for sink in q_accumulation_sound.iter() {
            sink.pause();
        }
    }
}

pub fn animate_jump(
    mut commands: Commands,
    mut jump_state: ResMut<JumpState>,
    time: Res<Time>,
    mut q_player: Query<&mut Transform, With<Player>>,
    game_sounds: Res<GameSounds>,
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
            if !jump_state.falled {
                commands.spawn(AudioBundle {
                    source: game_sounds.success.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
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
// TODO 蓄力过程中保持与平台相接触
pub fn animate_player_accumulation(
    accumulator: Res<Accumulator>,
    mut q_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = q_player.single_mut();
    match accumulator.0 {
        Some(_) => {
            player.scale.x = (player.scale.x + 0.12 * time.delta_seconds()).min(1.3);
            player.scale.y = (player.scale.y - 0.15 * time.delta_seconds()).max(0.6);
            player.scale.z = (player.scale.z + 0.12 * time.delta_seconds()).min(1.3);
        }
        None => {
            player.scale = Vec3::ONE;
        }
    }
}

pub fn animate_fall(
    mut commands: Commands,
    mut fall_state: ResMut<FallState>,
    jump_state: Res<JumpState>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut q_player: Query<&mut Transform, With<Player>>,
    game_sounds: Res<GameSounds>,
) {
    if !fall_state.completed && jump_state.completed {
        if !fall_state.played_sound {
            commands.spawn(AudioBundle {
                source: game_sounds.fall.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
            fall_state.played_sound = true;
        }
        let mut player = q_player.single_mut();
        match fall_state.fall_type {
            FallType::Straight => {
                if player.translation.y < 0.5 {
                    // 已摔落在地
                    fall_state.completed = true;
                    info!("Game over!");
                    next_game_state.set(GameState::GameOver);
                } else {
                    player.translation.y -= 0.7 * time.delta_seconds();
                }
            }
            FallType::Tilt(direction) => {
                if !fall_state.tilt_completed {
                    // 倾斜
                    let around_point = Vec3::new(
                        fall_state.pos.x,
                        INITIAL_PLAYER_POS.y - 0.5,
                        fall_state.pos.z,
                    );
                    if player.translation.y < around_point.y {
                        fall_state.tilt_completed = true;
                    } else {
                        let quat = Quat::from_axis_angle(
                            direction,
                            1.0 * FRAC_PI_2 * time.delta_seconds(),
                        );
                        player.rotate_around(around_point, quat);
                    }
                } else {
                    // 下坠
                    if player.translation.y < 0.2 {
                        // 已摔落在地
                        fall_state.completed = true;
                        info!("Game over!");
                        next_game_state.set(GameState::GameOver);
                    } else {
                        player.translation.y -= 0.7 * time.delta_seconds();
                    }
                }
            }
        }
    }
}

pub fn animate_accumulation_particle_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    accumulator: Res<Accumulator>,
    mut effect_timer: ResMut<GenerateAccumulationParticleEffectTimer>,
    time: Res<Time>,
    mut q_effect: Query<(Entity, &mut ParticleEffect, &mut Transform)>,
    q_player: Query<&Transform, (With<Player>, Without<ParticleEffect>)>,
) {
    if accumulator.0.is_some() {
        // 生成粒子特效
        effect_timer.0.tick(time.delta());
        if effect_timer.0.just_finished() {
            println!("LWZTEST spawn effect");
            let player = q_player.single();
            let mut color_gradient = Gradient::new();
            color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
            color_gradient.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
            color_gradient.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
            color_gradient.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

            let mut size_gradient = Gradient::new();
            size_gradient.add_key(0.0, Vec2::splat(0.05));
            size_gradient.add_key(0.3, Vec2::splat(0.05));
            size_gradient.add_key(1.0, Vec2::splat(0.0));

            let name = format!("accumulation{}", time.elapsed_seconds() as u32);
            let mut module = Module::default();

            let init_pos = SetPositionSphereModifier {
                center: module.lit(player.translation),
                radius: module.lit(1.0),
                dimension: ShapeDimension::Volume,
            };

            let lifetime = module.lit(2.); // literal value "10.0"
            let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

            let update_linear_drag = LinearDragModifier::constant(&mut module, 8.0);

            let update_force_field = ForceFieldModifier::new(vec![ForceFieldSource {
                position: player.translation,
                max_radius: 10.0,
                min_radius: 0.0,
                mass: 6.0,
                force_exponent: 0.3,
                conform_to_sphere: false,
            }]);

            let effect = effects.add(
                EffectAsset::new(3, Spawner::once(3.0.into(), true), module)
                    .init(init_pos)
                    .init(init_lifetime)
                    .update(update_linear_drag)
                    .update(update_force_field)
                    .render(ColorOverLifetimeModifier {
                        gradient: color_gradient.clone(),
                    })
                    .render(SizeOverLifetimeModifier {
                        gradient: size_gradient.clone(),
                        screen_space_size: false,
                    }),
            );
            commands.spawn((
                Name::new(name),
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect),
                    transform: Transform::IDENTITY,
                    ..Default::default()
                },
            ));

            effect_timer.0.reset();
        }
    } else {
        // 关闭粒子特效
        for (entity, _, _) in &mut q_effect {
            commands.entity(entity).despawn();
        }
    }
}

pub fn clear_player(mut commands: Commands, q_player: Query<Entity, With<Player>>) {
    for player in &q_player {
        commands.entity(player).despawn();
    }
}

pub fn prepare_jump(time: Res<Time>, mut prepare_timer: ResMut<PrepareJumpTimer>) {
    prepare_timer.0.tick(time.delta());
}

pub fn reset_prepare_jump_timer(mut prepare_timer: ResMut<PrepareJumpTimer>) {
    prepare_timer.0.reset();
}
