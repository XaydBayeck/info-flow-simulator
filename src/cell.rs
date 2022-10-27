use bevy::prelude::*;
use bevy_life::{CellState, CellularAutomatonPlugin, MooreCell2d};
use rand::Rng;

use crate::message::Message;

pub type CellRulePlugin = CellularAutomatonPlugin<MooreCell2d, Individual>;

#[derive(Component, Clone, Default)]
pub struct Individual {
    supported: Option<Message>,
    opposed: Vec<Message>,
    evaluatings: Vec<(usize, Message)>,
    p_send: f32,
}

impl Individual {
    pub fn infect(&mut self, msg: Message) {
        self.p_send = 0.5 * (msg.correlation + msg.interesting);
        self.supported = Some(msg);
    }

    fn add_eval_list(&mut self, msg: Message) {
        let (_, msgs): (Vec<_>, Vec<_>) = self.evaluatings.iter().cloned().unzip();
        if !msgs.contains(&msg) {
            self.evaluatings.push((msg.evaluate_times(), msg));
        }
    }

    fn recover_eval_list(&mut self) {
        for i in 0..self.evaluatings.len() {
            let (times, msg) = self.evaluatings[i].clone();

            // #[cfg(feature = "debug")]
            // info!("recovering: times = {:}", times);
            if times == 0 {
                self.evaluatings.remove(i);
                self.opposed.push(msg);
            }
        }
    }

    fn evaluate(&mut self, neighbors: &[Self]) {
        let mut rng = rand::thread_rng();
        let mut iter = self.evaluatings.iter_mut().enumerate();

        while let Some((idx, (times, msg))) = iter.next() {
            let ratio = (*times as f32 / msg.evaluate_times() as f32).powi(2) * msg.accept_ratio();

            #[cfg(feature = "debug")]
            info!(times = ?times, basic_p = msg.accept_ratio(), ratio = ?ratio);

            *times = *times - 1;
            let supportors = neighbors
                .iter()
                .filter(|&indv| {
                    if let Some(o_msg) = &indv.supported {
                        msg == o_msg
                    } else {
                        false
                    }
                })
                .count();

            let oppositers = neighbors
                .iter()
                .filter(|&indv| indv.opposed.contains(msg))
                .count();

            #[cfg(feature = "debug")]
            info!(supportors = supportors, oppositers = oppositers);

            let ratio = if supportors > oppositers {
                ((supportors - oppositers) as f32 / neighbors.len() as f32) * ratio
            } else {
                0.
            };

            let random: f32 = rng.gen();
            if random < ratio {
                self.supported = Some(msg.clone());
                self.evaluatings.remove(idx);
                break;
            }
        }

        // #[cfg(feature = "debug")]
        // if !self.evaluatings.is_empty() {
        //     info!("info_list = {:?}", self.evaluatings);
        // }
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        let supported_is_same = if let Some(o_msg) = &other.supported {
            !self.opposed.contains(&o_msg) && {
                match &self.supported {
                    None => false,
                    Some(msg) => msg.eq(&o_msg),
                }
            }
        } else {
            self.supported.is_none()
        };

        let evaluates_not_change = self.evaluatings.eq(&other.evaluatings);

        supported_is_same && evaluates_not_change
    }
}

impl CellState for Individual {
    fn color(&self) -> Option<bevy::render::color::Color> {
        self.supported
            .iter()
            .next()
            .map(|msg| msg.face)
            .or(Some(Color::WHITE))
    }

    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let mut rng = rand::thread_rng();

        // #[cfg(feature = "debug")]
        // if !self.evaluatings.is_empty() {
        //     info!("Before evaluate info_list = {:?}", self.evaluatings);
        // }

        let mut new_self = self.clone();

        // 加入评估列表
        for neighbor in neighbor_cells {
            if let Some(msg) = &neighbor.supported {
                let ratio: f32 = rng.gen();
                if ratio < neighbor.p_send {
                    new_self.add_eval_list(msg.clone());
                }
            }
        }

        // 评估信息
        new_self.evaluate(neighbor_cells);
        new_self.recover_eval_list();

        // DONE: 计算 Psend
        if let Some(msg) = &new_self.supported {
            new_self.p_send = 0.5 * (msg.correlation + msg.interesting);
        }

        // #[cfg(feature = "debug")]
        // if !new_self.evaluatings.is_empty() {
        //     info!("After evaluate info_list = {:?}", new_self.evaluatings);
        // }

        new_self
    }
}
