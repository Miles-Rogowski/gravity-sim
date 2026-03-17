use bevy::prelude::*;
use crate::hello_world::HelloWorldPlugin;
use crate::planet_creation::PlanetCreationPlugin;
use crate::simulation::SimulationPlugin;
use crate::controlls::ControllsPlugin;
use crate::ui::UIPlugin;

mod hello_world;
mod planet_creation;
mod simulation;
mod controlls;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloWorldPlugin)
        .add_plugins(PlanetCreationPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(ControllsPlugin)
        .add_plugins(UIPlugin)
        .add_systems(Startup, setup)
        //.add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands
){
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}