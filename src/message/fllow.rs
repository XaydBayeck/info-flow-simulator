use bevy::prelude::*;
use rand::Rng;

use crate::display::cell::{Cell, Coordinate};

use super::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum FllowLable {
    Send,
    Receive,
}

pub struct Deliver {
    sender_coor: Coordinate,
    message: Message,
}

pub fn receive_message(
    mut ev_deliver: EventReader<Deliver>,
    mut cells: Query<(&mut Cell, &mut Sprite, &Coordinate)>,
) {
    let mut rng = rand::thread_rng();
    for deliver in ev_deliver.iter() {
        for (mut cell, mut sprite, _) in cells
            .iter_mut()
            .filter(|(_, _, coor)| distance(&deliver.sender_coor, coor) < 2)
        {
            if rng.gen::<f32>() < deliver.message.accept_ratio() {
                cell.acc_message = Some(deliver.message.clone());
                sprite.color = deliver.message.face;
            }
        }
    }
}

pub fn send_message(mut ev_deliver: EventWriter<Deliver>, cells: Query<(&Cell, &Coordinate)>) {
    let mut rng = rand::thread_rng();

    for (cell, &coor) in cells.iter() {
        if let Some(msg) = cell.acc_message.clone() {
            if rng.gen::<f32>() < msg.spread_benefits() {
                ev_deliver.send(Deliver {
                    sender_coor: coor,
                    message: msg,
                })
            }
        }
    }
}

fn distance(lhs: &Coordinate, rhs: &Coordinate) -> usize {
    fn abs_sub(lhs: usize, rhs: usize) -> usize {
        if lhs > rhs {
            lhs - rhs
        } else {
            rhs - lhs
        }
    }

    abs_sub(lhs.0, rhs.0) + abs_sub(lhs.1, rhs.1)
}
