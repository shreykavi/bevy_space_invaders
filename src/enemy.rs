use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::{ActiveEnemies, Enemy, FromEnemy, Laser, Speed, WinSize, TIME_STEP};

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const SCALE: f32 = 0.8;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_laser_movement)
            .add_system(enemy_movement)
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
            .insert(Enemy)
            .insert(Speed::default());

        active_enemies.0 += 1;
    }
}

fn enemy_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Speed), With<Enemy>>) {
    let now = time.seconds_since_startup() as f32;

    // for each enemy
    for (mut tf, speed) in query.iter_mut() {
        let max_distance = TIME_STEP * speed.0;
        let x_org = tf.translation.x;
        let y_org = tf.translation.y;

        // Get the ellipse
        let (x_offset, y_offset) = (0., 100.);
        let (x_radius, y_radius) = (150., 100.);

        // Compute the next angle
        let angle = speed.0 * TIME_STEP * now % 360. / PI;

        // Calculate the destination
        let x_dst = x_radius * angle.cos() + x_offset;
        let y_dst = y_radius * angle.sin() + y_offset;

        // Calculate the distance
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance == 0. {
            0.
        } else {
            max_distance / distance
        };

        // Calculate final x/y (and not overshooting)
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // Apply translations
        tf.translation.x = x;
        tf.translation.y = y;
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
        if tf.translation.y < -win_size.h / 2. - 50. {
            commands.entity(entity).despawn();
        }
    }
}
