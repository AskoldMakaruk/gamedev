use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::WorldInspectorPlugin;

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
        .insert_resource(Msaa { samples: 4 })
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

fn startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let shape = shapes::Circle::default();
    commands.insert_resource(GameState::new());
    // camera
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    // commands
    //     .spawn(GeometryBuilder::build_as(
    //         &shape,
    //         DrawMode::Outlined {
    //             fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::RED),
    //             outline_mode: StrokeMode::new(Color::BLACK, 10.),
    //         },
    //         Transform::from_xyz(0.0, 400.0, 0.0).with_scale(Vec3::splat(1.)),
    //     ))
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7));

    // commands
    //     .spawn(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
    //     .insert(Collider::cuboid(500.0, 50.0));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
    physics_showoff(&mut commands, &mut materials);
}

fn physics_showoff(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    const TRANSPARENT_RED: Color = Color::rgba_linear(1.0, 0.0, 0.0, 0.5);
    const TRANSPARENT_GREEN: Color = Color::rgba_linear(0.0, 1.0, 0.0, 0.5);
    const TRANSPARENT_BLUE: Color = Color::rgba_linear(0.0, 0.0, 1.0, 0.5);
    const CIRCLE_RADIUS: f32 = 50.0;
    const PI_2: f32 = 2.0 * std::f32::consts::PI;
    let circle = shapes::Circle {
        radius: CIRCLE_RADIUS,
        ..shapes::Circle::default()
    };

    physics_static_geometry(commands);

    let circle_interaction_groups = InteractionGroups::new(0x0002.into(), 0x0001.into());
    let colors = vec![TRANSPARENT_RED, TRANSPARENT_GREEN, TRANSPARENT_BLUE];
    let num_colors = colors.len();
    for (i, color) in colors.into_iter().enumerate() {
        let x = CIRCLE_RADIUS * 1.5 * (PI_2 / num_colors as f32 * i as f32).cos();
        let y = CIRCLE_RADIUS * 1.5 * (PI_2 / num_colors as f32 * i as f32).sin();

        // let dir_x = (rand::random::<f32>() - 0.5) * 2.0; // rand number in [-1.0, 1.0]
        // let dir_y = (rand::random::<f32>() - 0.5) * 2.0;
        // let speed = (rand::random::<f32>() * 75.0).max(25.0);
        // let vel = Vec2::new(dir_x, dir_y).normalize() * speed;

        // let rigid_body = RigidBodyBuilder::new_dynamic()
        //     .translation(x + WINDOW_WIDTH, y)
        //     .linvel(vel.x, vel.y);
        // let collider = ColliderBuilder::ball(CIRCLE_RADIUS)
        //     .friction(0.0)
        //     .restitution(1.0)
        //     .collision_groups(circle_interaction_groups);

        commands
            .spawn(GeometryBuilder::build_as(
                &circle,
                DrawMode::Fill(bevy_prototype_lyon::prelude::FillMode::color(color)),
                Transform::default(), // Rapier overrides transform.
            ))
            .insert(Collider::ball(CIRCLE_RADIUS))
            .insert(RigidBody::Dynamic);
    }
}
fn physics_static_geometry(commands: &mut Commands) {
    const WIDTH: f32 = 100.;
    let wall_interaction_groups = InteractionGroups::new(0x0001.into(), 0x0002.into());
    commands.spawn(Collider::compound(vec![
        //bottom
        (Vec2::new(0.0, -360.5), 0., Collider::cuboid(640., WIDTH)),
        //top
        (Vec2::new(0.0, 360.5), 0., Collider::cuboid(640., WIDTH)),
        //left
        (Vec2::new(-600., 0.0), 0., Collider::cuboid(WIDTH, 360.)),
        //right
        (Vec2::new(600., 0.0), 0., Collider::cuboid(WIDTH, 360.)),
    ]));
    // screen botto    // // screen_left
    // let rigid_body = RigidBodyBuilder::new_static().translation(639.5, 0.0);
    // let collider = ColliderBuilder::cuboid(1.0, 360.0)
    //     .friction(0.0)
    //     .restitution(1.0)
    //     .collision_groups(wall_interaction_groups);
    // commands.spawn((rigid_body, collider));
    // // screen right
    // let rigid_body = RigidBodyBuilder::new_static().translation(1920.5, 0.0);
    // let collider = ColliderBuilder::cuboid(1.0, 360.0)
    //     .friction(0.0)
    //     .restitution(1.0)
    //     .collision_groups(wall_interaction_groups);
    // commands.spawn((rigid_body, collider));
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
