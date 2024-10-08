use core::f32;

use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle};

use crate::{
    asset_loader::SpriteAssets,
    collision::{Collider, CollisionDamage},
    health::Health,
    movement::{MovementBundle, Velocity},
    schedule::InGame,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("player")
            .add_systems(Update, throw_weapon.in_set(InGame::ProcessCombat))
            .add_systems(Update, player_movement.in_set(InGame::UserInput));
    }
}

const PLAYER_SPEED: f32 = 50.;
const PLAYER_SIZE: Vec2 = Vec2::splat(15.);
const PLAYER_START_HEALTH: u32 = 30;
const PLAYER_ATTACK_COOLDOWN: f32 = 1.5;
const PLAYER_DAMAGE_COOLDOWN: f32 = 0.25;
const DAGGER_SPEED: f32 = 100.;
const DAGGER_SPAWN_DISTANCE: f32 = 16.;
const DAGGER_DAMAGE: u32 = 5;
const DAGGER_HEALTH: u32 = 1;

#[derive(Component, Debug, Default)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Weapon(Timer);

#[derive(Component, Debug)]
pub struct Dagger;

#[derive(Bundle, Debug, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    health: Health,
    collider: Collider,
    weapon: Weapon,
    movement: MovementBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_sheet_bundle: Default::default(),
            movement: MovementBundle {
                velocity: Velocity::new(0., 0.),
            },
            weapon: Weapon(Timer::from_seconds(
                PLAYER_ATTACK_COOLDOWN,
                TimerMode::Repeating,
            )),
            collider: Collider::new(PLAYER_SIZE),
            health: Health::new(PLAYER_START_HEALTH),
        }
    }
}

fn player_movement(
    mut player_q: Query<(&mut Velocity, &mut Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut player_velocity, mut player_transform)) = player_q.get_single_mut() {
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
        player_transform.translation.z = 100.;
    }
}

fn throw_weapon(
    mut commands: Commands,
    mut player_q: Query<(&mut Weapon, &Transform), With<Player>>,
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
) {
    let Ok((mut weapon, player_transform)) = player_q.get_single_mut() else {
        return;
    };

    weapon.0.tick(time.delta());

    if weapon.0.just_finished() {
        for (i, direction) in [Vec3::Y, Vec3::NEG_X, Vec3::NEG_Y, Vec3::X]
            .into_iter()
            .enumerate()
        {
            let mut transform = *player_transform;

            transform.translation += direction * DAGGER_SPAWN_DISTANCE;
            transform.translation.z = 100.;
            transform.rotate(Quat::from_axis_angle(
                Vec3::Z,
                (i as f32) * std::f32::consts::FRAC_PI_2,
            ));

            commands.spawn((
                Dagger,
                SpriteBundle {
                    texture: sprites.dagger.clone(),
                    transform,
                    ..default()
                },
                Collider::new(Vec2::new(8., 13.)),
                CollisionDamage::new(DAGGER_DAMAGE),
                Health::with_damage_cooldown(DAGGER_HEALTH, PLAYER_DAMAGE_COOLDOWN),
                MovementBundle {
                    velocity: Velocity::from_direction_speed(direction, DAGGER_SPEED),
                },
            ));
        }
    }
}
