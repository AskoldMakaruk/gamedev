use std::ops::{Add, Mul};

use bevy::prelude::*;

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_system);
    }
}

#[derive(Component)]
pub struct Movable {
    pub speed: Vec2,
    pub face: Vec2,
    pub additional_rotation: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            speed: Vec2::ZERO,
            face: Vec2::Y,
            additional_rotation: 0.,
        }
    }
}

fn move_system(mut query: Query<(&mut Transform, &Movable)>, time: Res<Time>) {
    for (mut pos, mov) in query.iter_mut() {
        // move
        pos.translation = pos
            .translation
            .add(mov.speed.mul(250. * time.delta_seconds()).extend(0.));

        // rotate
        let target = mov.face.extend(pos.translation.z);
        pos.look_at(target, Vec3::NEG_Z);
        pos.rotate_z(mov.additional_rotation);
    }
}
