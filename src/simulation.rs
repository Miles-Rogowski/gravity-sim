use::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::planet_creation::*;
use crate::ui::SliderWidgetStates;
use std::collections::HashMap;
use std::collections::HashSet;
use::rand::*;


pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Update, update)
        .add_systems(Update, update_absorb_timers);
    }
}

struct CombinationEntity{
    entity: Entity,
    scale: f32,
    vel_x: f32,
    vel_y: f32,
    mass: f32,
    density: f32,
    x: f32,
    y: f32,
    debris_multiplier: i32
}

const MAX_DEBRIS_OFFSET: f32 = 0.5;
const MAX_DEBRIS_DIRECTION_OFFSET: f32 = 100.0;
const MIN_DEBRIS_MASS: f32 = 0.5;
const MAX_DEBRIS_MASS: f32 = 2.5;
const DEBRIS_PER_COLLISION: i32 = 10;

pub const GRAVITY_MULTIPLIER: f32 = 500.0;

fn update(
    mut planets: Query<(Entity, &Formed, &mut Velocity, &mut Transform, &Mass, &Sprite, &AbsorbTimer, &mut Scale), Without<Camera>>,
    mut commands: Commands,
    slider_values: Res<SliderWidgetStates>,
    camera: Query<(&Camera, &GlobalTransform, &Transform, &Projection)>,
    window: Query<&Window, With<PrimaryWindow>>,
){
    let mut rng = rand::rng();

    let Ok((_camera, _camera_transform, camera_position, projection)) = camera.single() else { panic!("no camera!") };
    let Projection::Orthographic(ref zoom) = *projection else { panic!("no projection!") };

    let window = window.single().unwrap();

    let width = window.width();
    let height = window.height();
    
    let mut accelerations: HashMap<Entity, Vec2> = HashMap::new();

    let mut combinations: HashMap<Entity, CombinationEntity> = HashMap::new();
    let mut entities_to_despawn: HashSet<Entity> = HashSet::new();

    if planets.iter().len() > 1 {
        for [a, b] in planets.iter_combinations::<2>(){

            let a_scale = (a.7.delta * TEXTURE_SIZE as f32 / 2.0) as f32;
            let b_scale = (b.7.delta * TEXTURE_SIZE as f32 / 2.0) as f32;


            let distance_x = a.3.translation.x - b.3.translation.x;
            let distance_y = a.3.translation.y - b.3.translation.y;

            let distance = distance_x * distance_x + distance_y * distance_y;

            if distance == 0.0{ 
                continue;
            }
            let mut force = 55.743*((a.4.mass*b.4.mass)/distance) * GRAVITY_MULTIPLIER * slider_values.sliders["Gravity Multiplier"].slider_value;

            if distance.sqrt() < a_scale + b_scale{
                //colliding
                force = 0.0;

                if !combinations.contains_key(&a.0) && !combinations.contains_key(&b.0) && !entities_to_despawn.contains(&a.0)  && !entities_to_despawn.contains(&b.0) && !combinations.contains_key(&b.0) && a.6.0 <= 0.0 && b.6.0 <= 0.0{

                    if a_scale > b_scale{
                        let b_pair = CombinationEntity{ entity: b.0, scale: b_scale, vel_x: b.2.x, vel_y: b.2.y, mass: b.4.mass, density: b.4.density, x: b.3.translation.x, y: b.3.translation.y, debris_multiplier: b.4.debris_multiplier};
                        combinations.insert(a.0, b_pair);
                        entities_to_despawn.insert(b.0);
                    }
                    else{
                        let a_pair = CombinationEntity{ entity: a.0, scale: a_scale, vel_x: a.2.x, vel_y: a.2.y, mass: a.4.mass, density: a.4.density, x: a.3.translation.x, y: a.3.translation.y, debris_multiplier: a.4.debris_multiplier};
                        combinations.insert(b.0, a_pair);
                        entities_to_despawn.insert(a.0);
                    }

                    
                }


                
            }


            let norm_x = distance_x / distance.sqrt();
            let norm_y = distance_y / distance.sqrt();

            if a.4.mass == 0.0 || b.4.mass == 0.0{
                continue;
            }


            let a_acceleration = accelerations.entry(a.0).or_insert(Vec2::ZERO);
            a_acceleration.x -= (force * norm_x / a.4.mass) * b.4.debris_multiplier as f32; // dont move a when b is debris
            a_acceleration.y -= (force * norm_y / a.4.mass) * b.4.debris_multiplier as f32; // dont move a when b is bebris

            let b_acceleration = accelerations.entry(b.0).or_insert(Vec2::ZERO);
            b_acceleration.x += (force * norm_x / b.4.mass) * a.4.debris_multiplier as f32; // dont move b when a is bebris
            b_acceleration.y += (force * norm_y / b.4.mass) * a.4.debris_multiplier as f32; // dont move b when a is bebris


        }
    }

    


    for mut planet in planets.iter_mut(){

        planet.2.x += accelerations.get(&planet.0).copied().unwrap_or(Vec2::ZERO).x;
        planet.2.y += accelerations.get(&planet.0).copied().unwrap_or(Vec2::ZERO).y;


        planet.3.translation.x += planet.2.x;
        planet.3.translation.y += planet.2.y;

        if combinations.contains_key(&planet.0){


            let r1 = planet.7.delta * TEXTURE_SIZE as f32 / 2.0;
            let r2 = combinations[&planet.0].scale;

            let r3 = (r1 * r1 + r2 * r2).sqrt();

            planet.7.delta = r3 / (TEXTURE_SIZE as f32 / 2.0);

            if planet.4.mass == 0.0 || combinations[&planet.0].mass == 0.0{
                continue;
            }

            if combinations[&planet.0].debris_multiplier != 0{
                planet.2.x = (planet.2.x * planet.4.mass + combinations[&planet.0].vel_x * combinations[&planet.0].mass) / (planet.4.mass + combinations[&planet.0].mass) as f32;
                planet.2.y = (planet.2.y * planet.4.mass + combinations[&planet.0].vel_y * combinations[&planet.0].mass) / (planet.4.mass + combinations[&planet.0].mass) as f32;
            }

            

            //create debris

            if combinations[&planet.0].mass > 5.0{
                for _i in 0..DEBRIS_PER_COLLISION{
                    let dx = combinations[&planet.0].x - planet.3.translation.x;
                    let dy = combinations[&planet.0].y - planet.3.translation.y;
                    let distance = (dx*dx + dy*dy).sqrt();

                    if distance == 0.0{
                        continue;
                    }

                    let x = (planet.3.translation.x + dx / distance * (planet.7.delta * TEXTURE_SIZE as f32 / 2.0)) + rng.random_range(-MAX_DEBRIS_OFFSET..MAX_DEBRIS_OFFSET);
                    let y = ( planet.3.translation.y + dy / distance * (planet.7.delta * TEXTURE_SIZE as f32 / 2.0)) + rng.random_range(-MAX_DEBRIS_OFFSET..MAX_DEBRIS_OFFSET);
                    let scale = (combinations[&planet.0].scale / 20.0) / (TEXTURE_SIZE as f32 / 2.0);
                    let vel_x = -combinations[&planet.0].vel_x / 2.5 + rng.random_range(-MAX_DEBRIS_DIRECTION_OFFSET  * scale..MAX_DEBRIS_DIRECTION_OFFSET  * scale);
                    let vel_y = -combinations[&planet.0].vel_y / 2.5 + rng.random_range(-MAX_DEBRIS_DIRECTION_OFFSET  * scale..MAX_DEBRIS_DIRECTION_OFFSET  * scale);
                    let mass = rng.random_range(MIN_DEBRIS_MASS..MAX_DEBRIS_MASS);
                    let density = combinations[&planet.0].density;
                    


                    commands.spawn((
                        Formed{},
                        Sprite{ image: planet.5.image.clone(), ..default()},
                        Transform::from_xyz(x, y, 5.0),
                        Velocity{ x: vel_x, y: vel_y, start_x: vel_x, start_y: vel_y },
                        Mass{ mass: mass, density: density, debris_multiplier: 0 },
                        Scale{ delta: scale },
                        AbsorbTimer( 5.0 )
                    ));
                }
                
            }
        }

    }

    for (_a, b) in combinations{
        if !entities_to_despawn.contains(&b.entity){
            entities_to_despawn.insert(b.entity);
            commands.entity(b.entity).despawn();
        }
        
    }

    for entity in &entities_to_despawn{
        commands.entity(*entity).despawn();
    }

    //despawn entities outside screen * 2

    for planet in planets{
        if !entities_to_despawn.contains(&planet.0){
            if planet.3.translation.x > width * zoom.scale + camera_position.translation.x{
                commands.entity(planet.0).despawn();
            }
            else if planet.3.translation.x < -width * zoom.scale + camera_position.translation.x{
                commands.entity(planet.0).despawn();
            }

            if planet.3.translation.y > height * zoom.scale + camera_position.translation.y{
                commands.entity(planet.0).despawn();
            }
            else if planet.3.translation.y < -height * zoom.scale + camera_position.translation.y{
                commands.entity(planet.0).despawn();
            }
        }
    }

}


fn update_absorb_timers(
    mut timers: Query<&mut AbsorbTimer>,
    time: Res<Time>
){
    for mut timer in timers.iter_mut(){
        if timer.0 > 0.0{
            timer.0 -= 15.0 * time.delta_secs();
        }else{
            timer.0 = 0.0;
        }
    }
}
