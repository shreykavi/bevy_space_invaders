use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

use crate::{ActiveEnemies, Enemy, FromEnemy, Laser, Speed, WinSize, TIME_STEP};

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const SCALE: f32 = 0.8;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_laser_movement)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(enemy_spawn),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.9))
                    .with_system(enemy_fire),
            );
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
) {
    if active_enemies.0 < 1 {
        // compute the random position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        // spawn the enemy
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    // x,y,z
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SCALE, SCALE, 1.),
                    ..Default::default()
                },
                texture: asset_server.load(ENEMY_SPRITE),
                ..Default::default()
            })
            .insert(Enemy);

        active_enemies.0 += 1;
    }
}

fn enemy_fire(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_query: Query<(&Transform, With<Enemy>)>,
) {
    // For each enemy shoot laser
    for (&tf, _) in enemy_query.iter() {
        let x = tf.translation.x;
        let y = tf.translation.y;

        // spawn enemy laser sprite
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    // x,y,z
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(SCALE, -SCALE, 1.),
                    ..Default::default()
                },
                texture: asset_server.load(ENEMY_LASER_SPRITE),
                ..Default::default()
            })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(Speed::default());
    }
}

fn enemy_laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut laser_query: Query<(
        Entity,
        &Speed,
        &mut Transform,
        (With<Laser>, With<FromEnemy>),
    )>,
) {
    for (entity, speed, mut tf, _) in laser_query.iter_mut() {
        tf.translation.y -= speed.0 * TIME_STEP;
        println!("{:?}", tf.translation);
        if tf.translation.y < -win_size.h / 2. - 50. {
            commands.entity(entity).despawn();
        }
    }
}
