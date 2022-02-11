use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

use crate::{ActiveEnemies, Enemy, WinSize};

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const SCALE: f32 = 1.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn),
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
                // sprite: Sprite {
                //     custom_size: Some(Vec2::new(200., 100.)),
                //     color: Color::rgb(1., 0.7, 0.7),
                //     ..Default::default()
                // },
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
