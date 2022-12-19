use std::ops::{Add, Mul};

use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_system).add_system(rotate_system);
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

fn move_system(mut query: Query<(&mut KinematicCharacterController, &Movable)>, time: Res<Time>) {
    for (mut pos, mov) in query.iter_mut() {
        pos.translation = Some(
            pos.translation
                .unwrap_or(default())
                .add(mov.speed.mul(250. * time.delta_seconds())),
        );
    }
}

fn rotate_system(mut query: Query<(&mut Transform, &Movable)>) {
    for (mut pos, mov) in query.iter_mut() {
        let target = mov.face.extend(pos.translation.z);
        pos.look_at(target, Vec3::NEG_Z);
        pos.rotate_z(mov.additional_rotation);
    }
}
