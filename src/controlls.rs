use::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::*;
use::rand::*;
use crate::planet_creation::*;

pub struct ControllsPlugin;

impl Plugin for ControllsPlugin {
    fn build(&self, app: &mut App){
        app
        .insert_resource(Zoom(20.0))
        .insert_resource(MouseInertia{ x: 0.0, y: 0.0 })
        .add_systems(Update, keyboard_shortcuts);
    }
}

pub const ZOOM_SCALE: f32 = 0.05;

#[derive(Component)]
struct ActivePlanet{
    x_offset: f32,
    y_offset: f32
}

#[derive(Resource)]
pub struct Zoom(pub f32);

#[derive(Resource)]
struct MouseInertia{
    x: f32,
    y: f32
}


fn keyboard_shortcuts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut planets: Query<(Entity, &mut Transform, &Scale, &mut Position, &mut Velocity, Option<&ActivePlanet>)>,
    mut zoom: ResMut<Zoom>,
    mut mouse_inertia: ResMut<MouseInertia>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
){

    let mut rng = rand::rng();

    let window = window.single().unwrap();


    let width = window.width();
    let height = window.height();

    //spawn random planets
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        for _i in 0..20{
            let x = rng.random_range(-width / 2.0 / (zoom.0 * ZOOM_SCALE)..width / 2.0 / (zoom.0 * ZOOM_SCALE));
            let y = rng.random_range(-height / 2.0 / (zoom.0 * ZOOM_SCALE)..height / 2.0) / (zoom.0 * ZOOM_SCALE);

            let vel_x = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);
            let vel_y = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);

            let scale = rng.random_range(1.0..50.0);
            
            let color = PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())];

            let dens = rng.random_range(MIN_DENSITY..MAX_DENSITY);

            let mass = dens * scale;

            commands.spawn((
                Formed{},
                Mesh2d(meshes.add(Circle::new(1.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(color)))),
                Transform::from_xyz(x, y, 5.0),//.with_scale(Vec3{ x: scale, y: scale, z: 10.0 }),
                Velocity{ x: vel_x, y: vel_y },
                Mass{ mass: mass, density: dens, debris_multiplier: 1 },
                Scale{ delta: mass/dens * 2.0 },
                Position{ x: x, y: y },
                AbsorbTimer( 0.0 )
            ));
        }
        
    }

    if mouse_input.just_pressed(MouseButton::Middle){
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.single() {
                let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
                for mut planet in planets.iter_mut(){

                    let dx = world_pos.x - planet.1.translation.x;
                    let dy = world_pos.y - planet.1.translation.y;
                    let r = planet.1.scale.x;

                    if dx * dx + dy * dy <= r * r{
                        commands.entity(planet.0).insert(ActivePlanet{ x_offset: dx, y_offset: dy });
                    }
                }
            }
        }
    }

    if mouse_input.pressed(MouseButton::Middle){
        mouse_inertia.x = mouse_motion.delta.x;
        mouse_inertia.y = mouse_motion.delta.y;

        //drag planet

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.single() {
                let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();

                for mut planet in planets.iter_mut(){
                    if planet.5.is_some(){
                        planet.4.x = 0.0;
                        planet.4.y = 0.0;
                        planet.3.x = (world_pos.x - planet.5.unwrap().x_offset) / (zoom.0 * ZOOM_SCALE);
                        planet.3.y = (world_pos.y - planet.5.unwrap().y_offset) / (zoom.0 * ZOOM_SCALE);
                    }
                }
            
            }
        }
    }
    else{
        if mouse_inertia.x != 0.0 && mouse_inertia.y != 0.0{
            //add to planet velocity

            if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.single() {
                let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();

                for mut planet in planets.iter_mut(){
                    if planet.5.is_some(){
                        planet.4.x = 0.0;
                        planet.4.y = 0.0;
                        planet.4.x += mouse_inertia.x / (zoom.0 * ZOOM_SCALE);
                        planet.4.y -= mouse_inertia.y / (zoom.0 * ZOOM_SCALE);

                        commands.entity(planet.0).remove::<ActivePlanet>();
                    }
                }
            
            }
        }

            

        }

        mouse_inertia.x = 0.0;
        mouse_inertia.y = 0.0;
    }


    //panning
    if mouse_input.pressed(MouseButton::Right){
        for mut planet in planets.iter_mut(){
            planet.3.x += mouse_motion.delta.x / (zoom.0 * ZOOM_SCALE);
            planet.3.y -= mouse_motion.delta.y / (zoom.0 * ZOOM_SCALE);
        }
    }


    //zooming
    zoom.0 += mouse_scroll.delta.y;

    if zoom.0 < 1.0{
        zoom.0 = 1.0;
    }
    else if zoom.0 > 40.0{
        zoom.0 = 40.0;
    }

    for mut planet in planets.iter_mut(){
        planet.1.scale.x = planet.2.delta * zoom.0 * ZOOM_SCALE;
        planet.1.scale.y = planet.2.delta * zoom.0 * ZOOM_SCALE;

        planet.1.translation.x = planet.3.x * zoom.0 * ZOOM_SCALE;
        planet.1.translation.y = planet.3.y * zoom.0 * ZOOM_SCALE;
    }


}