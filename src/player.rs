use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let cam: &Transform = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera:,  {}", e)).unwrap(),
        };

        let mut direction: Vec3 = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        if keys.pressed(KeyCode::S) {
            direction -= cam.forward();
        }

        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        direction.y = 0.0;

        let movement = direction.normalize_or_zero() * time.delta_seconds() * player_speed.0;

        player_transform.translation += movement;

        // Rotate player to face camera
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }

    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let flashlight = SpotLightBundle::default();

    let player = (
        SceneBundle{
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed(5.0),
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}
