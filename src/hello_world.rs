use::bevy::prelude::*;

pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, setup);
    }
}

fn setup(){
    println!("Hello World!");
}