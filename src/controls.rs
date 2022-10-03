use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;
use leafwing_input_manager::prelude::*;

pub struct Controls;

impl Plugin for Controls {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system(movement);
    }
}

#[derive(Actionlike, Clone)]
pub enum Action {
    Accelerate,
    TurnRight,
}

fn movement(mut player: Query<(&ActionState<Action>, &mut ExternalForce, &mut Transform)>) {
    player.for_each_mut(|(action_state, mut force, mut trans)| {
        if action_state.pressed(Action::Accelerate) {
            force.force = trans.forward() * 2.0;
        }
        if action_state.pressed(Action::TurnRight) {
            trans.rotate_local_y(-0.01);
        }
    })
}
