use crate::{
    collision::Collider,
    ghost::Ghost,
    player::{Dagger, Player},
    schedule::InGame,
};
use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                take_damage::<Player, Ghost>,
                take_damage::<Ghost, Dagger>,
                despawn_dead_entities,
            )
                .chain()
                .in_set(InGame::ProcessCombat),
        );
    }
}

const DAMAGE_COOLDOWN: f32 = 0.25;

#[derive(Component, Debug)]
pub struct Health {
    amount: u32,
    pub cooldown: Timer,
}

impl Health {
    pub fn new(amount: u32) -> Self {
        let mut cooldown = Timer::from_seconds(DAMAGE_COOLDOWN, TimerMode::Once);
        cooldown.pause();

        Self { amount, cooldown }
    }
}

fn take_damage<T: Component, E: Component>(
    mut player_q: Query<(&mut Health, &Collider), With<T>>,
    enemies: Query<&E, With<Collider>>,
    time: Res<Time>,
) {
    for (mut health, collider) in player_q.iter_mut() {
        let can_be_damaged = collider.collisions.iter().any(|e| enemies.contains(*e));

        if can_be_damaged && health.cooldown.paused() {
            health.cooldown.unpause();
        }

        health.cooldown.tick(time.delta());

        if can_be_damaged && health.cooldown.finished() {
            health.cooldown.reset();
            health.amount = health.amount.saturating_sub(1);
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    entities_q: Query<(Entity, &Health), Without<Player>>,
) {
    for (entity, health) in entities_q.iter() {
        if health.amount == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
