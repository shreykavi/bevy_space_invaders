use bevy::prelude::*;

use crate::{FromPlayer, Laser, Player, PlayerReadyFire, Speed, WinSize, TIME_STEP};

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_players", SystemStage::single(player_spawn))
            .add_system(player_movement)
            .add_system(player_fire)
            .add_system(laser_movement);
    }
}

fn player_spawn(mut commands: Commands, asset_server: Res<AssetServer>, win_size: Res<WinSize>) {
    // spawn a sprite
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 100.)),
                color: Color::rgb(1., 0.7, 0.7),
                ..Default::default()
            },
            transform: Transform {
                // x,y,z
                translation: Vec3::new(0., bottom + 75. / 4. + 20., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            texture: asset_server.load(PLAYER_SPRITE),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyFire(true))
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    let (speed, mut transform, _) = query.single_mut();
    {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>,
) {
    let (player_tf, mut ready_fire, _) = query.single_mut();
    if ready_fire.0 && keyboard_input.pressed(KeyCode::Space) {
        let x = player_tf.translation.x;
        let y = player_tf.translation.y;

        let mut spawn_lasers = |x_offset: f32| {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        // x,y,z
                        translation: Vec3::new(x + x_offset, y + 10., 0.),
                        ..Default::default()
                    },
                    texture: asset_server.load(PLAYER_LASER_SPRITE),
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(Speed::default());
        };

        // Width of Ferris for lasers
        let x_offset = 144. / 4. + 5.;
        spawn_lasers(x_offset);
        spawn_lasers(-x_offset);

        ready_fire.0 = false;
    }

    if keyboard_input.just_released(KeyCode::Space) {
        ready_fire.0 = true;
    }
}

fn laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(
        Entity,
        &Speed,
        &mut Transform,
        (With<Laser>, With<FromPlayer>),
    )>,
) {
    for (laser_entity, speed, mut laser_tf, _) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y += speed.0 * TIME_STEP;
        if translation.y > win_size.h {
            // Despawning is extremely important otherwise the movement will
            // infinitely be calculated
            commands.entity(laser_entity).despawn();
        }
    }
}
