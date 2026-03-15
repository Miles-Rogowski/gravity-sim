use::bevy::prelude::*;
use crate::planet_creation::*;
use std::collections::HashMap;


pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, setup)
        .add_systems(Update, update);
    }
}

fn setup(){
    println!("simulation test");
}

fn update(
    mut planets: Query<(Entity, &Formed, &mut Velocity, &mut Transform, &Mass)>
){

    let mut accelerations: HashMap<Entity, Vec2> = HashMap::new();

    if planets.iter().len() > 1 {
        for [a, b] in planets.iter_combinations::<2>(){
            
            //acceleration = force/mass
            //force = 55.743((mass1*mass2)/distance^2)


            let distance_x = a.3.translation.x - b.3.translation.x;
            let distance_y = a.3.translation.y - b.3.translation.y;

            let distance = distance_x * distance_x + distance_y * distance_y;

            let mut force = 55.743*((a.4.mass*b.4.mass)/distance);

            if distance < a.3.scale.x * 2.0{
                force = 0.0;
            }


            let norm_x = distance_x / distance.sqrt();
            let norm_y = distance_y / distance.sqrt();


            let a_acceleration = accelerations.entry(a.0).or_insert(Vec2::ZERO);
            a_acceleration.x -= (force * norm_x) / a.4.mass;
            a_acceleration.y -= (force * norm_y) / a.4.mass;

            let b_acceleration = accelerations.entry(b.0).or_insert(Vec2::ZERO);
            b_acceleration.x += (force * norm_x) / b.4.mass;
            b_acceleration.y += (force * norm_y) / b.4.mass;

            /*a.2.x += (force * norm_x) / a.4.mass;
            b.2.x -= (force * norm_x) / b.4.mass;

            a.2.y += (force * norm_y) / a.4.mass;
            b.2.y -= (force * norm_y) / b.4.mass;*/

        }
    }

    


    for mut planet in planets.iter_mut(){

        planet.2.x += accelerations.get(&planet.0).copied().unwrap_or(Vec2::ZERO).x;
        planet.2.y += accelerations.get(&planet.0).copied().unwrap_or(Vec2::ZERO).y;


        planet.3.translation.x += planet.2.x;
        planet.3.translation.y += planet.2.y;
    }
}