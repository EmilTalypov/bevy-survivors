use bevy::prelude::*;

use crate::{
    collision::{Collider, CollisionEvent},
    ghost::Ghost,
    levels::WallTile,
    player::Player,
    schedule::InGame,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_position,
                keep_inside_walls::<Player>,
                keep_inside_walls::<Ghost>,
            )
                .chain()
                .in_set(InGame::EntityUpdate),
        );
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

fn keep_inside_walls<T: Component>(
    mut events: EventReader<CollisionEvent>,
    mut entities_q: Query<(&mut Transform, &Collider), (With<T>, Without<WallTile>)>,
    walls_q: Query<(&Transform, &Collider), With<WallTile>>,
) {
    for event in events.read() {
        let Ok((mut entity_transform, entity_collider)) = entities_q.get_mut(event.entity) else {
            continue;
        };

        let Ok((wall_transform, wall_collider)) = walls_q.get(event.collided_with) else {
            continue;
        };

        let entity_rect = entity_collider.to_rect_at(&entity_transform);
        let wall_rect = wall_collider.to_rect_at(&wall_transform);

        let overlap = entity_rect.intersect(wall_rect);

        let base_push = if overlap.width() < overlap.height() {
            Vec2::new(overlap.width(), 0.)
        } else {
            Vec2::new(0., overlap.height())
        };

        let push_away = base_push * (entity_rect.center() - wall_rect.center()).signum();
        entity_transform.translation += push_away.extend(0.);
    }
}
