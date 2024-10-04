use bevy::prelude::*;

use crate::{collision::Collider, player::Player, schedule::InGame};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, take_damage.in_set(InGame::ProcessCombat));
    }
}

#[derive(Component, Debug)]
pub struct Health {
    amount: u32,
    pub cooldown: Timer,
}

impl Health {
    pub fn new(amount: u32) -> Self {
        let mut cooldown = Timer::from_seconds(0.25, TimerMode::Once);
        cooldown.pause();

        Self { amount, cooldown }
    }
}

fn take_damage(mut player_q: Query<(&mut Health, &Collider), With<Player>>, time: Res<Time>) {
    let Ok((mut health, collider)) = player_q.get_single_mut() else {
        return;
    };

    if !collider.collisions.is_empty() && health.cooldown.paused() {
        health.cooldown.unpause();
    }

    health.cooldown.tick(time.delta());

    if !collider.collisions.is_empty() && health.cooldown.finished() {
        health.cooldown.reset();
        health.amount = health.amount.saturating_sub(1);
    }
}
