use core::f32;

use bevy::prelude::*;

use crate::{
    asset_loader::SpriteAssets,
    collision::Collider,
    health::Health,
    movement::{MovementBundle, Velocity},
    schedule::InGame,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, throw_weapon.in_set(InGame::ProcessCombat))
            .add_systems(Update, player_movement.in_set(InGame::UserInput));
    }
}

const PLAYER_SPEED: f32 = 150.;
const PLAYER_SIZE: Vec2 = Vec2::splat(15.);
const PLAYER_START_HEALTH: u32 = 30;
const PLAYER_ATTACK_COOLDOWN: f32 = 1.;
const DAGGER_SPEED: f32 = 25.;
const DAGGER_SPAWN_DISTANCE: f32 = 16.;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Weapon(Timer);

#[derive(Component, Debug)]
pub struct Dagger;

fn spawn_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: sprites.knight.clone(),
            ..default()
        },
        Player,
        MovementBundle {
            velocity: Velocity::new(0., 0.),
        },
        Weapon(Timer::from_seconds(
            PLAYER_ATTACK_COOLDOWN,
            TimerMode::Repeating,
        )),
        Collider::new(PLAYER_SIZE),
        Health::new(PLAYER_START_HEALTH),
    ));
}

fn player_movement(
    mut player_q: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut player_velocity) = player_q.get_single_mut() {
        let mut direction = Vec3::ZERO;

        for key in keyboard_input.get_pressed() {
            match key {
                KeyCode::KeyW => direction += Vec3::Y,
                KeyCode::KeyA => direction += Vec3::NEG_X,
                KeyCode::KeyS => direction += Vec3::NEG_Y,
                KeyCode::KeyD => direction += Vec3::X,
                _ => {}
            }
        }

        player_velocity.change_direction_speed(direction, PLAYER_SPEED);
    }
}

fn throw_weapon(
    mut commands: Commands,
    mut player_q: Query<(&mut Weapon, &Transform), With<Player>>,
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
) {
    let (mut weapon, player_transform) = player_q.single_mut();

    weapon.0.tick(time.delta());

    if weapon.0.just_finished() {
        for (i, direction) in [Vec3::Y, Vec3::NEG_X, Vec3::NEG_Y, Vec3::X]
            .into_iter()
            .enumerate()
        {
            let mut transform = *player_transform;

            transform.translation += direction * DAGGER_SPAWN_DISTANCE;
            transform.rotate(Quat::from_axis_angle(
                Vec3::Z,
                (i as f32) * f32::consts::FRAC_PI_2,
            ));

            commands.spawn((
                Dagger,
                SpriteBundle {
                    texture: sprites.dagger.clone(),
                    transform,
                    ..default()
                },
                Collider::new(Vec2::new(8., 13.)),
                MovementBundle {
                    velocity: Velocity::from_direction_speed(direction, DAGGER_SPEED),
                },
            ));
        }
    }
}
