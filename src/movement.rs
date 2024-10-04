use bevy::prelude::*;

use crate::schedule::InGame;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position.in_set(InGame::EntityUpdate));
    }
}

#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn from_direction_speed(direction: Vec3, speed: f32) -> Self {
        Self {
            value: direction.normalize_or_zero() * speed,
        }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: Vec3::new(x, y, 0.),
        }
    }

    pub fn change_direction_speed(&mut self, direction: Vec3, speed: f32) {
        self.value = direction.normalize_or_zero() * speed;
    }

    // pub fn accelerate(&mut self, acceleration: Vec3, dt: f32) {
    //     self.value += acceleration * dt;
    // }
}

#[derive(Bundle, Default, Debug)]
pub struct MovementBundle {
    pub velocity: Velocity,
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
