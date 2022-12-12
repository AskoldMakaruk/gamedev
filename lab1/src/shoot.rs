use bevy::{asset::*, prelude::*, reflect::TypeUuid};
use bevy_inspector_egui::Inspectable;

use crate::Movable;

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_bullet);
        app.add_system(despawn_bullets);

        let mut assets = app.world.resource_mut::<Assets<StandardMaterial>>();
        let _ = assets.set_untracked(
            BULLET_MATERIAL,
            StandardMaterial {
                base_color: Color::Rgba {
                    red: 0.7,
                    green: 0.2,
                    blue: 0.05,
                    alpha: 1.0,
                },
                ..default()
            },
        );

        let mut assets = app.world.resource_mut::<Assets<Mesh>>();
        let _ = assets.set_untracked(BULLET_MESH, shape::Box::default().into());
    }
}

pub const BULLET_MATERIAL: HandleUntyped =
    HandleUntyped::weak_from_u64(StandardMaterial::TYPE_UUID, 929205183514206030);

pub const BULLET_MESH: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 815363642349823395);

fn spawn_bullet(mut commands: Commands, mut turrets: Query<(&Transform, &mut Turret, &Movable)>) {
    for (pos, mut turret, mov) in turrets.iter_mut() {
        let offset = match turret.shoot() {
            Some(of) => of,
            None => continue,
        };

        let direction = mov.face - pos.translation.truncate();
        let translation = (pos.rotation.mul_vec3(offset.extend(0.))) + pos.translation;
        let material = BULLET_MATERIAL.typed::<StandardMaterial>();
        let mesh = BULLET_MESH.typed::<Mesh>();

        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform {
                    scale: Vec3::new(15.0, 5., 20.0),
                    translation,
                    ..default()
                },
                ..default()
            },
            Bullet {
                speed: 300.,
                life: 600,
            },
            Movable {
                speed: direction.normalize(),
                face: mov.face,
                ..default()
            },
        ));
    }
}

fn despawn_bullets(mut commands: Commands, mut bullets: Query<(&mut Bullet, Entity)>) {
    for (mut b, entity) in bullets.iter_mut() {
        b.life -= 1;

        if b.life <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub life: i32,
}

#[derive(Component, Reflect, Inspectable)]
pub struct Turret {
    pub bullet_spawn_offsets: Vec<Vec2>,
    pub ticks_to_shoot: i32,
    pub ticks_since_shot: i32,
    pub is_shooting: bool,
    pub total_shot: usize,
}

impl Turret {
    fn tick(&mut self) {
        self.ticks_since_shot += 1;
    }
    pub fn shoot(&mut self) -> Option<Vec2> {
        self.tick();
        if self.ticks_to_shoot <= self.ticks_since_shot && self.is_shooting {
            self.ticks_since_shot = 0;
            self.total_shot += 1;
            Some(self.choose_bullet_offset())
        } else {
            None
        }
    }
    fn choose_bullet_offset(&self) -> Vec2 {
        self.bullet_spawn_offsets[self.total_shot % self.bullet_spawn_offsets.len()]
    }
}

impl Default for Turret {
    fn default() -> Self {
        Self {
            bullet_spawn_offsets: vec![Vec2::new(25., 30.), Vec2::new(-25., 30.)],
            ticks_to_shoot: 30,
            ticks_since_shot: 0,
            is_shooting: false,
            total_shot: 0,
        }
    }
}
