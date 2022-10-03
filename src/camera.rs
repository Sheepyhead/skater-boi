use bevy::prelude::*;

use crate::Player;

const CAMERA_OFFSET: [f32; 3] = [0.0, 2.0, 3.0];

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera3dBundle::default();

    camera.transform.translation = CAMERA_OFFSET.into();
    camera.transform.look_at(Vec3::NEG_Z, Vec3::Y);

    commands.spawn_bundle(camera);
}

fn follow_player(
    mut camera: Query<&mut Transform, With<Camera3d>>,
    player: Query<&Transform, (With<Player>, Changed<Transform>, Without<Camera3d>)>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();

    camera.translation = player.translation + Vec3::from(CAMERA_OFFSET);
}
