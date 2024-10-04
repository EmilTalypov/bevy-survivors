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
            .add_systems(Update, player_movement.in_set(InGame::UserInput));
    }
}

const PLAYER_SPEED: f32 = 50.;
const PLAYER_SIZE: Vec2 = Vec2::splat(15.);
const PLAYER_START_HEALTH: u32 = 30;

#[derive(Component, Debug)]
pub struct Player;

fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite_assets.knight.clone(),
            ..default()
        },
        Player,
        MovementBundle {
            velocity: Velocity::new(0., 0.),
        },
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
