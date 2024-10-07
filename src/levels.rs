use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::collision::Collider;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_systems(Startup, load_levels)
            .add_systems(Update, add_wall_colliders);
    }
}

pub const TILE_SIZE: i32 = 16;
pub const TILE_DIMENSIONS: IVec2 = IVec2::splat(TILE_SIZE);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

#[derive(Bundle, Debug, Default, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

fn load_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..default()
    });
}

fn add_wall_colliders(
    mut commands: Commands,
    walls_q: Query<(Entity, &TileEnumTags), Added<Wall>>,
) {
    for (entity, enum_tags) in walls_q.iter() {
        let mut entity_commands = commands.entity(entity);

        for tag in &enum_tags.tags {
            match tag.as_ref() {
                "Top" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        TILE_DIMENSIONS.as_vec2(),
                        Vec2::new(0., 12.), // not working
                    ));
                }
                "Left" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        TILE_DIMENSIONS.as_vec2(),
                        Vec2::new(-2., 0.),
                    ));
                }
                "Right" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        TILE_DIMENSIONS.as_vec2(),
                        Vec2::new(2., 0.),
                    ));
                }
                "Bottom" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        TILE_DIMENSIONS.as_vec2(),
                        Vec2::new(0., -2.),
                    ));
                }
                _ => {}
            }
        }
    }
}
