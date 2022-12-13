use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::{camera::RenderTarget, mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, SpriteAssetEvents},
};
use bevy_prototype_lyon::prelude::*;
use std::ops::*;

use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub mod mouse;
pub use mouse::*;

pub mod shoot;
pub use shoot::*;

pub mod player;
pub use player::*;

pub mod walk;
pub use walk::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(ShootPlugin)
        .add_plugin(MovePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(startup)
        .add_event::<MouseMoveEvent>()
        .add_system(my_cursor_system)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn startup(mut commands: Commands) {
    commands.insert_resource(GameState::new());
    // camera
    commands
        .spawn(Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
                scale: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

#[derive(Resource)]
pub struct GameState {
    pub hp: i32,
    pub score: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self { hp: 0, score: 0 }
    }
}
