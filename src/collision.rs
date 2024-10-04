use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::schedule::InGame;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions.in_set(InGame::CollisionDetection));
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub collisions: Vec<Entity>,
}

impl Collider {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            collisions: vec![],
        }
    }
}

fn detect_collisions(mut colliders_q: Query<(Entity, &Transform, &mut Collider)>) {
    let mut collisions: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in colliders_q.iter() {
        let rect_a = Rect::from_center_size(transform_a.translation.truncate(), collider_a.size);

        for (entity_b, transform_b, collider_b) in colliders_q.iter() {
            if entity_a == entity_b {
                continue;
            }

            let rect_b =
                Rect::from_center_size(transform_b.translation.truncate(), collider_b.size);

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

// fn knock_back_player(
//     mut player_q: Query<(&Collider, &Transform, &mut Velocity), With<Player>>,
//     transforms: Query<&Transform>,
//     time: Res<Time>,
// ) {
//     let Ok((collider, transform, mut velocity)) = player_q.get_single_mut() else {
//         return;
//     };

//     let mut knock_back = Vec3::ZERO;

//     for collision in collider.collisions.iter() {
//         let enemy_transform = transforms
//             .get(*collision)
//             .expect("Collided with entity without transform");

//         let direction = (transform.translation - enemy_transform.translation)
//             .try_normalize()
//             .unwrap_or(Vec3::NEG_X);

//         let acceleration = direction * 30.;

//         knock_back += acceleration;
//     }

//     velocity.accelerate(knock_back, time.elapsed_seconds());
// }
