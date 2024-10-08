use crate::{
    collision::{CollisionDamage, CollisionEvent},
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
        )
        .add_systems(Update, tick_damage_cooldown.in_set(InGame::EntityUpdate));
    }
}

#[derive(Component, Debug)]
pub struct Health {
    amount: u32,
    pub cooldown: Option<f32>,
}

#[derive(Component, Debug)]
pub struct DamageCooldown {
    pub cooldown: Timer,
}

impl DamageCooldown {
    fn new(cooldown: f32) -> Self {
        Self {
            cooldown: Timer::from_seconds(cooldown, TimerMode::Once),
        }
    }
}

impl Health {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            cooldown: None,
        }
    }

    pub fn with_damage_cooldown(amount: u32, cooldown: f32) -> Self {
        Self {
            amount,
            cooldown: Some(cooldown),
        }
    }
}

fn take_damage<T: Component, E: Component>(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut reciever_q: Query<&mut Health, (With<T>, Without<DamageCooldown>)>,
    damager_q: Query<&CollisionDamage, With<E>>,
) {
    for collision in events.read() {
        let Ok(mut health) = reciever_q.get_mut(collision.entity) else {
            continue;
        };

        let Ok(damage) = damager_q.get(collision.collided_with) else {
            continue;
        };

        health.amount = health.amount.saturating_sub(damage.amount);

        if let Some(duration) = health.cooldown.as_ref() {
            commands
                .entity(collision.entity)
                .insert(DamageCooldown::new(*duration));
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, entities_q: Query<(Entity, &Health)>) {
    for (entity, health) in entities_q.iter() {
        if health.amount == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn tick_damage_cooldown(
    mut commands: Commands,
    mut entities_q: Query<(Entity, &mut DamageCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in entities_q.iter_mut() {
        timer.cooldown.tick(time.delta());

        if timer.cooldown.just_finished() {
            commands.entity(entity).remove::<DamageCooldown>();
        }
    }
}
