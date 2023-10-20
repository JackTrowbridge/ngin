use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod player;
mod world;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
