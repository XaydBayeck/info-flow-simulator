use bevy::prelude::*;
use rand::Rng;

use crate::display::cell::{distance, Cell, Coordinate, Neighbors};

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

pub fn send_message(
    mut ev_deliver: EventWriter<Deliver>,
    cells: Query<(&Cell, &Coordinate, &Neighbors)>,
) {
    let mut rng = rand::thread_rng();

    for (cell, &coor, neighbors) in cells.iter() {
        if let Some(msg) = cell.acc_message.clone() {
            let len = neighbors.0.len() as f32;
            let knowned = neighbors.0.iter().fold(0, |acc, id| {
                cells
                    .get(id.clone())
                    .map(|(cell_n, _, _)| {
                        if {
                            match &cell_n.acc_message {
                                None => false,
                                Some(x) => (|msg_n: &Message| msg_n.face == msg.face)(x),
                            }
                        } {
                            acc
                        } else {
                            acc + 1
                        }
                    })
                    .or::<()>(Ok(acc))
                    .unwrap()
            }) as f32;

            if rng.gen::<f32>() < msg.spread_benefits() * knowned / len {
                ev_deliver.send(Deliver {
                    sender_coor: coor,
                    message: msg,
                })
            }
        }
    }
}
