use crate::{
    asset_loader::SpriteAssets,
    collision::Collider,
    health::Health,
    movement::{MovementBundle, Velocity},
    player::Player,
    schedule::InGame,
};
use bevy::prelude::*;
use rand::Rng;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>()
            .add_systems(Update, spawn_ghost)
            .add_systems(Update, chase_player.in_set(InGame::EntityUpdate));
    }
}

const GHOST_SPEED: f32 = 30.;
const SPAWN_INTERVAL: f32 = 1.;
const GHOST_SIZE: Vec2 = Vec2::splat(15.);

#[derive(Component, Debug)]
pub struct Ghost;

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

fn spawn_ghost(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    window_q: Query<&Window>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::thread_rng();

        let window = window_q.get_single().unwrap();

        let angle = rng.gen_range((0.)..(std::f32::consts::TAU));
        let unit_vector = Vec2::from_angle(angle);

        let vector_on_square = unit_vector / unit_vector.x.abs().max(unit_vector.y.abs());

        let spawn_point =
            vector_on_square.extend(0.) * Vec3::new(window.width() / 2., window.height() / 2., 0.);

        let direction = Vec3::ZERO - spawn_point;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(spawn_point),
                texture: sprite_assets.ghost.clone(),
                ..default()
            },
            Ghost,
            Health::new(1),
            MovementBundle {
                velocity: Velocity::from_direction_speed(direction, GHOST_SPEED),
            },
            Collider::new(GHOST_SIZE),
        ));
    }
}

fn chase_player(
    mut ghosts_q: Query<(&mut Velocity, &Transform), With<Ghost>>,
    player_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        for (mut ghost_velocity, ghost_transform) in ghosts_q.iter_mut() {
            let direction = player_transform.translation - ghost_transform.translation;

            ghost_velocity.change_direction_speed(direction, GHOST_SPEED);
        }
    }
}
