use bevy::prelude::*;

mod player;
mod camera;
mod world;

use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, WorldPlugin))
    .add_systems(Update, bevy::window::close_on_esc)
    .run();
}
