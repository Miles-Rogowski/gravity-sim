use::bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, setup);
    }
}

fn setup(){
    println!("simulation test");
}