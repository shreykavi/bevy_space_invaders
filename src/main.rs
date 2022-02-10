#![allow(unused)] // Silences any unused warnings

mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

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
        .add_plugin(PlayerPlugin)
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
