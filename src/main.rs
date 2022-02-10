#![allow(unused)] // Silences any unused warnings

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;

//  ECS

// Resources:
struct WinSize {
    w: f32,
    h: f32,
}

// Components:
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Laser;

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_stage("game_setup_players", SystemStage::single(player_spawn))
        .add_system(player_movement)
        .add_system(player_fire)
        .add_system(laser_movement)
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
                    texture: asset_server.load(LASER_SPRITE),
                    ..Default::default()
                })
                .insert(Laser)
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
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>,
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
