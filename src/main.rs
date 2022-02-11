#![allow(unused)] // Silences any unused warnings

mod enemy;
mod player;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

const TIME_STEP: f32 = 1. / 60.;

//  ECS

// Resources:
struct WinSize {
    w: f32,
    h: f32,
}

struct ActiveEnemies(u32);

// Components:
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Laser;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct PlayerReadyFire(bool);

#[derive(Component)]
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup)
        .add_system(laser_hit_enemy)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // Watches for changes in files
    asset_server.watch_for_changes().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window to top left
    let mut window = windows.get_primary_mut().unwrap();

    // Creates a resource that can later be used
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // window.set_position(IVec2::new(1600,200));
}

fn laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>,
) {
    // Had to hardcode sizes since bevy 0.6 no longer provides property
    let laser_size = Vec2::new(9., 54.);
    let enemy_size = Vec2::new(84., 93.);
    for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
            // truncate throws away the z coordinate
            let laser_scale = Vec3::truncate(laser_tf.scale);
            let enemy_scale = Vec3::truncate(enemy_tf.scale);

            let collision = collide(
                laser_tf.translation,
                laser_size * laser_scale,
                enemy_tf.translation,
                enemy_size * enemy_scale,
            );

            if let Some(_) = collision {
                // enemy dies
                commands.entity(enemy_entity).despawn();
                active_enemies.0 -= 1;

                // remove the laser
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
