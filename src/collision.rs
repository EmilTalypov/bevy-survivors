use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    ghost::Ghost,
    player::{Dagger, Player},
    schedule::InGame,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions.in_set(InGame::CollisionDetection))
            .add_systems(
                Update,
                (
                    handle_collisions::<Player>,
                    handle_collisions::<Ghost>,
                    handle_collisions::<Dagger>,
                )
                    .in_set(InGame::ProcessCombat),
            )
            .add_event::<CollisionEvent>();
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_with: Entity,
}

impl CollisionEvent {
    fn new(entity: Entity, collided_with: Entity) -> Self {
        Self {
            entity,
            collided_with,
        }
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
    pub collisions: Vec<Entity>,
}

impl Default for Collider {
    fn default() -> Self {
        Self::new(Vec2::ZERO)
    }
}

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: u32,
}

impl CollisionDamage {
    pub fn new(amount: u32) -> Self {
        Self { amount }
    }
}

impl Collider {
    pub fn new(size: Vec2) -> Self {
        Self::with_size_and_offset(size, Vec2::ZERO)
    }

    pub fn with_size_and_offset(size: Vec2, offset: Vec2) -> Self {
        Self {
            size,
            offset,
            collisions: vec![],
        }
    }

    pub fn to_rect_at(&self, transform: &Transform) -> Rect {
        Rect::from_center_size(transform.translation.truncate() + self.offset, self.size)
    }
}

fn detect_collisions(mut colliders_q: Query<(Entity, &Transform, &mut Collider)>) {
    let mut collisions: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in colliders_q.iter() {
        let rect_a = collider_a.to_rect_at(transform_a);

        for (entity_b, transform_b, collider_b) in colliders_q.iter() {
            if entity_a == entity_b {
                continue;
            }

            let rect_b = collider_b.to_rect_at(transform_b);

            if !rect_a.intersect(rect_b).is_empty() {
                collisions.entry(entity_a).or_default().push(entity_b);
            }
        }
    }

    for (entity_a, _, mut collider_a) in colliders_q.iter_mut() {
        collider_a.collisions.clear();
        collider_a.collisions = collisions.remove(&entity_a).unwrap_or_default();
    }
}

fn handle_collisions<T: Component>(
    mut events: EventWriter<CollisionEvent>,
    entities_q: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in entities_q.iter() {
        for collided_with in collider.collisions.iter() {
            events.send(CollisionEvent::new(entity, *collided_with));
        }
    }
}
