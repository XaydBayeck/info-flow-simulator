use bevy::prelude::*;

use crate::{cell::Individual, CELL_SIZE, MAX_EVALUATE_TIMES};

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let test_message = Message {
            correlation: 0.6,
            readability: 0.6,
            ambiguity: 0.8,
            rationality: 0.4,
            interesting: 0.6,
            explainability: 0.6,
            face: Color::BLACK,
        };

        app.insert_resource(test_message)
            .add_system(cursor_change_cell_color);
    }
}

/// 在人群之间传播的信息
#[derive(Debug, Clone, Component, Reflect, Default)]
pub struct Message {
    /// 相关性
    pub correlation: f32,
    /// 易读性
    pub readability: f32,
    /// 模糊性
    pub ambiguity: f32,
    /// 合理性
    pub rationality: f32,
    /// 趣味性
    pub interesting: f32,
    /// 可解释性
    pub explainability: f32,
    pub face: Color,
}

impl Message {
    pub fn accept_ratio(&self) -> f32 {
        const W: [f32; 4] = [0.22, 0.603, 0.11, 0.067];
        let es = [
            self.correlation,
            self.readability,
            self.rationality,
            self.interesting,
        ];

        es.into_iter()
            .zip(W.into_iter())
            .map(|(pe, w)| pe * w)
            .sum()
    }

    pub fn evaluate_times(&self) -> usize {
        (self.correlation * MAX_EVALUATE_TIMES as f32 + 0.5).floor() as usize
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.face.eq(&other.face)
    }
}

fn cursor_change_cell_color(
    windows: Res<Windows>,
    message: Res<Message>,
    buttons: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut cells: Query<(&mut Individual, &GlobalTransform)>,
) {
    if buttons.pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();

        let wnd = windows.get_primary().unwrap();

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            cells
                .iter_mut()
                .filter(|(_, &trans)| {
                    let cell_pos = trans.translation().truncate();
                    cell_pos.distance(world_pos) <= CELL_SIZE / 2.
                })
                .for_each(|(mut cell, _)| {
                    // sprite.color = message.face;
                    cell.infect(message.to_owned());
                });
        }
    }
}
