#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::enum_glob_use
)]

use bevy::{prelude::*, window::PresentMode};
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use controls::{Action, Controls};
use debug::Debug;
use leafwing_input_manager::{prelude::InputMap, InputManagerBundle};

mod camera;
mod controls;
mod debug;

pub const CLEAR: Color = Color::BLACK;
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Skater Boi".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // External plugins
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // Internal plugins
        .add_plugin(CameraPlugin)
        .add_plugin(Controls)
        .add_plugin(Debug)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_ground)
        .run();
}


fn spawn_ground(mut commands: Commands) {
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert_bundle((Collider::cuboid(100.0, 0.1, 100.0), RigidBody::Fixed));
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpatialBundle { ..default() })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: InputMap::new([
                (KeyCode::Comma, Action::Accelerate),
                (KeyCode::E, Action::TurnRight),
            ]),
            ..default()
        })
        .insert_bundle((
            Collider::cylinder(0.5, 0.25),
            RigidBody::Dynamic,
            ExternalForce::default(),
            LockedAxes::ROTATION_LOCKED,
            Player,
        ));
}

#[derive(Component)]
pub struct Player;
