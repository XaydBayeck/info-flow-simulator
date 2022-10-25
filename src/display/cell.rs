use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};

use crate::{message::Message, WorldSize, BORDER, CELL_SIZE};

#[derive(Component, Default)]
pub struct Cell {
    pub acc_message: Option<Message>,
}

#[derive(Component)]
pub struct Neighbors(pub Vec<Entity>);

#[derive(Component)]
pub struct Cells;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate(pub usize, pub usize);

#[derive(Bundle)]
pub struct CellBundle {
    cell: Cell,
    coor: Coordinate,
    #[bundle]
    sprite: SpriteBundle,
}

impl CellBundle {
    pub fn new(size: f32, col: usize, row: usize) -> Self {
        Self {
            cell: Cell::default(),
            coor: Coordinate(col, row),
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    size * col as f32,
                    size * row as f32,
                    0.,
                )),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(CELL_SIZE - BORDER)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn setup_map(mut commands: Commands, world_size: Res<WorldSize>) {
    let (cols, rows) = (world_size.col(), world_size.row());

    let cells = (0..(rows * cols))
        .map(|idx| {
            let (col, row) = (idx / rows, idx % rows);
            (
                commands
                    .spawn_bundle(CellBundle::new(CELL_SIZE, col, row))
                    .id(),
                Coordinate(col, row),
            )
        })
        .collect::<Vec<_>>();

    let cells = cells.clone().into_iter().map(|(cell, coor)| {
        let neighbors = cells
            .iter()
            .filter(|(_, coorn)| distance(&coor, coorn) < 3)
            .map(|(id, _)| id.clone())
            .collect();
        commands.entity(cell).insert(Neighbors(neighbors)).id()
    }).collect::<Vec<_>>();

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            -(CELL_SIZE * cols as f32) / 2.,
            -(CELL_SIZE * rows as f32) / 2.,
            0.,
        )))
        .insert(Cells)
        .push_children(&cells);
}

pub fn distance(lhs: &Coordinate, rhs: &Coordinate) -> usize {
    fn abs_sub(lhs: usize, rhs: usize) -> usize {
        if lhs > rhs {
            lhs - rhs
        } else {
            rhs - lhs
        }
    }

    abs_sub(lhs.0, rhs.0) + abs_sub(lhs.1, rhs.1)
}
