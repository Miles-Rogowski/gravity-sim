use::bevy::prelude::*;
use crate::planet_creation::*;
use std::collections::HashMap;
use std::collections::HashSet;
use::rand::*;


pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, setup)
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

const MAX_DEBRIS_OFFSET: f32 = 5.0;
const MAX_DEBRIS_DIRECTION_OFFSET: f32 = 5.0;
const MIN_DEBRIS_MASS: f32 = 0.5;
const MAX_DEBRIS_MASS: f32 = 2.5;
const DEBRIS_PER_COLLISION: i32 = 10;



fn setup(){
    println!("simulation test");
}

fn update(
    mut planets: Query<(Entity, &Formed, &mut Velocity, &mut Transform, &Mass, &MeshMaterial2d<ColorMaterial>, &AbsorbTimer)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
){
    let mut rng = rand::rng();

    let mut accelerations: HashMap<Entity, Vec2> = HashMap::new();

    let mut combinations: HashMap<Entity, CombinationEntity> = HashMap::new();
    let mut entities_to_despawn: HashSet<Entity> = HashSet::new();

    if planets.iter().len() > 1 {
        for [a, b] in planets.iter_combinations::<2>(){
            
            //acceleration = force/mass
            //force = 55.743((mass1*mass2)/distance^2)


            let distance_x = a.3.translation.x - b.3.translation.x;
            let distance_y = a.3.translation.y - b.3.translation.y;

            let distance = distance_x * distance_x + distance_y * distance_y;

            let mut force = 55.743*((a.4.mass*b.4.mass)/distance);

            if distance.sqrt() < a.3.scale.x + b.3.scale.x{
                //colliding
                force = 0.0;

                if !combinations.contains_key(&a.0) && !combinations.contains_key(&b.0) && !entities_to_despawn.contains(&a.0)  && !entities_to_despawn.contains(&b.0) && !combinations.contains_key(&b.0) && a.6.0 <= 0.0 && b.6.0 <= 0.0{

                    if a.4.mass > b.4.mass{
                        let b_pair = CombinationEntity{ entity: b.0, scale: b.3.scale.x, vel_x: b.2.x, vel_y: b.2.y, mass: b.4.mass, density: b.4.density, x: b.3.translation.x, y: b.3.translation.x, debris_multiplier: b.4.debris_multiplier};
                        combinations.insert(a.0, b_pair);
                        entities_to_despawn.insert(b.0);
                    }
                    else{
                        let a_pair = CombinationEntity{ entity: a.0, scale: a.3.scale.x, vel_x: a.2.x, vel_y: a.2.y, mass: a.4.mass, density: b.4.density, x: b.3.translation.x, y: b.3.translation.x, debris_multiplier: b.4.debris_multiplier};
                        combinations.insert(b.0, a_pair);
                        entities_to_despawn.insert(a.0);
                    }

                    
                }


                
            }


            let norm_x = distance_x / distance.sqrt();
            let norm_y = distance_y / distance.sqrt();


            let a_acceleration = accelerations.entry(a.0).or_insert(Vec2::ZERO);
            a_acceleration.x -= (force * norm_x) / a.4.mass * b.4.debris_multiplier as f32; // dont move a when b is debris
            a_acceleration.y -= (force * norm_y) / a.4.mass * b.4.debris_multiplier as f32; // dont move a when b is bebris

            let b_acceleration = accelerations.entry(b.0).or_insert(Vec2::ZERO);
            b_acceleration.x += (force * norm_x) / b.4.mass;
            b_acceleration.y += (force * norm_y) / b.4.mass;


        }
    }

    


    for mut planet in planets.iter_mut(){

        planet.2.x += accelerations.get(&planet.0).copied().unwrap_or(Vec2::ZERO).x;
        planet.2.y += accelerations.get(&planet.0).copied().unwrap_or(Vec2::ZERO).y;


        planet.3.translation.x += planet.2.x;
        planet.3.translation.y += planet.2.y;

        if combinations.contains_key(&planet.0){
            let r1 = planet.3.scale.x;
            let r2 = combinations[&planet.0].scale;

            let r3 = (r1 * r1 + r2 * r2).sqrt();

            planet.3.scale.x = r3;
            planet.3.scale.y = r3;

            planet.2.x += combinations[&planet.0].vel_x / combinations[&planet.0].mass * combinations[&planet.0].debris_multiplier as f32;
            planet.2.y += combinations[&planet.0].vel_y / combinations[&planet.0].mass * combinations[&planet.0].debris_multiplier as f32;

            //create debris

            if combinations[&planet.0].mass > 10.0{
                for _i in 0..DEBRIS_PER_COLLISION{
                    let x = (planet.3.translation.x + combinations[&planet.0].x) / 2.0 + rng.random_range(-MAX_DEBRIS_OFFSET..MAX_DEBRIS_OFFSET);
                    let y = (planet.3.translation.y + combinations[&planet.0].y) / 2.0 + rng.random_range(-MAX_DEBRIS_OFFSET..MAX_DEBRIS_OFFSET);
                    let vel_x = -combinations[&planet.0].vel_x + rng.random_range(-MAX_DEBRIS_DIRECTION_OFFSET..MAX_DEBRIS_DIRECTION_OFFSET);
                    let vel_y = -combinations[&planet.0].vel_y + rng.random_range(-MAX_DEBRIS_DIRECTION_OFFSET..MAX_DEBRIS_DIRECTION_OFFSET);
                    let mass = rng.random_range(MIN_DEBRIS_MASS..MAX_DEBRIS_MASS);
                    let density = combinations[&planet.0].density;

                    commands.spawn((
                        Formed{},
                        Mesh2d(meshes.add(Circle::new(1.0))),
                        MeshMaterial2d(planet.5.0.clone()),
                        Transform::from_xyz(x, y, 5.0).with_scale(Vec3{x: mass/density * 2.0, y: mass/density * 2.0, z: 1.0}),
                        Velocity{ x: vel_x, y: vel_y },
                        Mass{ mass: mass, density: density, debris_multiplier: 0 },
                        AbsorbTimer( 5.0 )
                    ));
                }
                
            }
        }

    }

    for (_a, b) in combinations{
        commands.entity(b.entity).despawn();
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