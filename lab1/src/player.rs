use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::{MouseMoveEvent, Movable, Turret};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot)
            .add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(rotate_player);
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_empty()
        .insert(PbrBundle {
            mesh: meshes.add(shape::Icosphere::default().into()).into(),
            material: materials.add(Color::ORANGE_RED.into()),
            transform: Transform::from_scale(Vec3::splat(30.)),
            ..Default::default()
        })
        .insert((Player, Turret::default()))
        .insert(Movable {
            additional_rotation: std::f32::consts::PI,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    mesh: meshes.add(create_triangle()),
                    material: materials.add(Color::AZURE.into()),
                    transform: Transform::from_translation(Vec3::new(-0.2, 1.1, 1.2))
                        .with_scale(Vec3::splat(0.2)),
                    ..default()
                })
                .insert(PlayerCursour);
        });
}

fn create_triangle() -> Mesh {
    let vertices = [
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 2.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];

    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

pub fn move_player(mut player_query: Query<&mut Movable, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut delta = Vec2::default();
    if keys.pressed(KeyCode::W) {
        delta += Vec2::Y;
    }
    if keys.pressed(KeyCode::D) {
        delta += Vec2::X;
    }
    if keys.pressed(KeyCode::S) {
        delta += Vec2::NEG_Y;
    }
    if keys.pressed(KeyCode::A) {
        delta += Vec2::NEG_X;
    }

    let mut player = player_query.single_mut();
    player.speed = delta;
}

#[derive(Component)]
struct PlayerCursour;

pub fn rotate_player(
    mut reader: EventReader<MouseMoveEvent>,
    mut player_query: Query<&mut Movable, With<Player>>,
) {
    let input = match reader.iter().last() {
        Some(s) => s,
        None => return,
    };

    for mut mov in player_query.iter_mut() {
        mov.face = input.pos;
    }
}

pub fn shoot(buttons: Res<Input<MouseButton>>, mut player: Query<&mut Turret, With<Player>>) {
    let is_pressed = buttons.pressed(MouseButton::Left);
    for mut p in player.iter_mut() {
        p.is_shooting = is_pressed;
    }
}
