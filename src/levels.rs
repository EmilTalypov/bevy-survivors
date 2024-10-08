use std::collections::{HashMap, HashSet};

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

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct WallTile;

#[derive(Bundle, Debug, Default, LdtkIntCell)]
pub struct WallBundle {
    pub wall: WallTile,
}

fn load_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..default()
    });
}

fn add_wall_colliders(
    mut commands: Commands,
    wall_q: Query<(&GridCoords, &Parent, Option<&TileEnumTags>), Added<WallTile>>,
    parent_q: Query<&Parent, Without<WallTile>>,
    level_q: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    let mut level_to_wall_locations: HashMap<Entity, HashMap<GridCoords, HashSet<String>>> =
        HashMap::new();

    wall_q
        .iter()
        .for_each(|(&grid_coords, parent, tile_enum_tags)| {
            if let Ok(grandparent) = parent_q.get(parent.get()) {
                let tags: HashSet<String> = tile_enum_tags
                    .map(|t| t.tags.clone().into_iter().collect())
                    .unwrap_or_default();

                level_to_wall_locations
                    .entry(grandparent.get())
                    .or_default()
                    .insert(grid_coords, tags);
            }
        });

    if !wall_q.is_empty() {
        level_q.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                let mut plate_stack: Vec<Vec<(Plate, HashSet<String>)>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<(Plate, HashSet<String>)> = Vec::new();
                    let mut plate_start = None;

                    for x in 0..width + 1 {
                        match (plate_start, level_walls.get(&GridCoords { x, y })) {
                            (Some((s, tags)), None) => {
                                row_plates.push((
                                    Plate {
                                        left: s,
                                        right: x - 1,
                                    },
                                    tags,
                                ));
                                plate_start = None;
                            }
                            (None, Some(tags)) => plate_start = Some((x, tags.clone())),
                            (Some((s, existing_tags)), Some(tags)) => {
                                plate_start =
                                    Some((s, existing_tags.union(tags).cloned().collect()));
                            }
                            _ => plate_start = None,
                        }
                    }

                    plate_stack.push(row_plates);
                }

                let mut rect_builder: HashMap<Plate, (Rect, HashSet<String>)> = HashMap::new();
                let mut prev_row: Vec<(Plate, HashSet<String>)> = Vec::new();
                let mut wall_rects: Vec<(Rect, HashSet<String>)> = Vec::new();

                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for (prev_plate, _) in &prev_row {
                        if !current_row
                            .iter()
                            .any(|(currect_plate, _)| prev_plate == currect_plate)
                        {
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }

                    for (plate, tags) in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|(e, t)| {
                                e.top += 1;
                                t.extend(tags.clone());
                            })
                            .or_insert((
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                                tags.clone(),
                            ));
                    }

                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    for (wall_rect, wall_tags) in wall_rects {
                        let mut width = (wall_rect.right - wall_rect.left + 1) * grid_size;
                        let mut height = (wall_rect.top - wall_rect.bottom + 1) * grid_size;
                        let mut offset = Vec2::ZERO;

                        for tag in &wall_tags {
                            match tag.as_ref() {
                                "left" => {
                                    width -= 10;
                                    offset.x -= 2.;
                                }
                                "right" => {
                                    width -= 10;
                                    offset.x += 2.;
                                }
                                "top" => {
                                    height -= 4;
                                    offset.y += 4.;
                                }
                                "bottom" => {
                                    height -= 4;
                                    offset.y -= 4.;
                                }
                                _ => (),
                            }
                        }

                        level.spawn((
                            WallTile,
                            Collider::with_size_and_offset(
                                IVec2::new(width, height).as_vec2(),
                                offset,
                            ),
                            SpatialBundle {
                                transform: Transform::from_xyz(
                                    ((wall_rect.left + wall_rect.right + 1) * grid_size) as f32
                                        / 2.,
                                    ((wall_rect.bottom + wall_rect.top + 1) * grid_size) as f32
                                        / 2.,
                                    0.,
                                ),
                                ..default()
                            },
                        ));
                    }
                });
            }
        });
    }
}
