#![allow(unused)] // Silences any unused warnings

mod enemy;
mod player;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use std::collections::HashSet;

const EXPLOSION_SHEET: &str = "explo_a_sprite.png";
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
struct Laser;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct PlayerReadyFire(bool);
#[derive(Component)]
struct FromPlayer;

#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct FromEnemy;

#[derive(Component)]
struct Explosion;
#[derive(Component)]
struct ExplosionToSpawn(Vec3);

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
        .add_system(player_laser_hit_enemy)
        .add_system(enemy_laser_hit_enemy)
        .add_system(explosion_to_spawn)
        .add_system(animate_explosion)
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

fn player_laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, (With<Laser>, With<FromPlayer>))>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, (With<Enemy>))>,
    mut active_enemies: ResMut<ActiveEnemies>,
) {
    let mut enemies_blasted: HashSet<Entity> = HashSet::new();

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
                if enemies_blasted.get(&enemy_entity).is_none() {
                    // enemy dies
                    commands.entity(enemy_entity).despawn();
                    active_enemies.0 -= 1;

                    // spawn explosion
                    commands
                        .spawn()
                        .insert(ExplosionToSpawn(enemy_tf.translation.clone()));

                    enemies_blasted.insert(enemy_entity);
                }

                // remove the laser
                commands.entity(laser_entity).despawn();
            }
        }
    }
}

fn enemy_laser_hit_enemy(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &Sprite), (With<Laser>, With<FromEnemy>)>,
    mut player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
) {
    // Had to hardcode sizes since bevy 0.6 no longer provides property
    let laser_size = Vec2::new(9., 54.);
    let player_size = Vec2::new(144., 75.);
    if let Ok((player_entity, player_tf, player_sprite)) = player_query.get_single_mut() {
        for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
            let laser_scale = Vec3::truncate(laser_tf.scale);
            let player_scale = Vec3::truncate(player_tf.scale);

            let collision = collide(
                laser_tf.translation,
                laser_size, //* laser_scale,
                player_tf.translation,
                player_size * player_scale,
            );

            if let Some(_) = collision {
                // player dies
                commands.entity(player_entity).despawn();

                // remove the laser
                commands.entity(laser_entity).despawn();

                // spawn explosion
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(player_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (entity, explosion_to_spawn) in query.iter() {
        let texture_handle = asset_server.load(EXPLOSION_SHEET);
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(64.0, 64.0),
                    4,
                    4,
                )),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(Timer::from_seconds(0.05, true));

        commands.entity(entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        With<Explosion>,
    )>,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle, _) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index == texture_atlas.textures.len() {
                commands.entity(entity).despawn()
            }
        }
    }
}
