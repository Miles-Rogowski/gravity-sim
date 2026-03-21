use::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::*;
use::rand::*;
use crate::planet_creation::*;

pub struct ControllsPlugin;

impl Plugin for ControllsPlugin {
    fn build(&self, app: &mut App){
        app
        .insert_resource(MouseInertia{ x: 0.0, y: 0.0 })
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_shortcuts)
        .add_systems(PostUpdate, lock_camera);
    }
}



#[derive(Component)]
struct PlanetReticle;

#[derive(Component)]
pub struct ActivePlanet{
    x_offset: f32,
    y_offset: f32
}

#[derive(Component)]
struct CameraLocked;


#[derive(Resource)]
struct MouseInertia{
    x: f32,
    y: f32
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let img = asset_server.load("outline.png");

    commands.spawn((
        PlanetReticle{},
        Transform::from_xyz(0.0, 0.0, 500.0).with_scale(Vec3{x: 10.0, y: 10.0, z: 10.0}),
        Sprite{ image: img, ..default()},
        Visibility::Hidden,
    ));
}


fn keyboard_shortcuts(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut planets: Query<(Entity, &mut Transform, &Scale, &mut Velocity, Option<&ActivePlanet>, Option<&CameraLocked>), Without<Camera>>,
    mut mouse_inertia: ResMut<MouseInertia>,
    mut camera: Query<(&Camera, &GlobalTransform, &mut Transform, &mut Projection)>,
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

    let Ok((camera, camera_transform, mut camera_position, mut projection)) = camera.single_mut() else { panic!("no camera!") };
    let Projection::Orthographic(ref mut zoom) = *projection else { panic!("no projection!") };

    //spawn random planets
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        for _i in 0..20{
            let x = rng.random_range(-width / 2.0..width / 2.0) * zoom.scale + camera_position.translation.x;
            let y = rng.random_range(-height / 2.0..height / 2.0) * zoom.scale + camera_position.translation.y;

            let vel_x = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);
            let vel_y = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);

            let scale = rng.random_range(1.0..50.0);
            
            let texture = generate_planet_texture(TEXTURE_SIZE, TEXTURE_SIZE, (TEXTURE_SIZE / 2) as f32, PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())], PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())]);

            let dens = rng.random_range(MIN_DENSITY..MAX_DENSITY);

            let mass = dens * scale;

            commands.spawn((
                Formed{},
                Sprite{ image: images.add(texture), ..default() },
                Transform::from_xyz(x, y, 5.0),
                Velocity{ x: vel_x, y: vel_y, start_x: vel_x, start_y: vel_y },
                Mass{ mass: mass, density: dens, debris_multiplier: 1 },
                Scale{ delta: scale },
                AbsorbTimer( 0.0 )
            ));
        }
        
    }

    if mouse_input.just_pressed(MouseButton::Middle){
        if let Some(cursor_pos) = window.cursor_position() {
            let cursor_pos_world = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
            for planet in planets.iter(){

                let dx = cursor_pos_world.x - planet.1.translation.x;
                let dy = cursor_pos_world.y - planet.1.translation.y;
                let r = (planet.1.scale.x * TEXTURE_SIZE as f32 / 2.0) as f32;

                if dx * dx + dy * dy <= r * r && !planet.5.is_some(){
                    commands.entity(planet.0).insert(ActivePlanet{ x_offset: dx, y_offset: dy });
                }
            }
        }
    }

    if mouse_input.pressed(MouseButton::Middle){
        mouse_inertia.x = mouse_motion.delta.x;
        mouse_inertia.y = mouse_motion.delta.y;

        //drag planet

        if let Some(cursor_pos) = window.cursor_position() {
            let cursor_pos_world = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
            for mut planet in planets.iter_mut(){
                if planet.4.is_some(){
                    planet.3.x = 0.0;
                    planet.3.y = 0.0;
                    planet.1.translation.x = cursor_pos_world.x - planet.4.unwrap().x_offset;
                    planet.1.translation.y = cursor_pos_world.y - planet.4.unwrap().y_offset;
                }
            }
        
        }
}
    else{
        //add to planet velocity
        for mut planet in planets.iter_mut(){
            if planet.4.is_some(){
                if mouse_inertia.x != 0.0 && mouse_inertia.y != 0.0{
                    planet.3.x += mouse_inertia.x * zoom.scale / 2.0;
                    planet.3.y -= mouse_inertia.y * zoom.scale / 2.0;
                }

                commands.entity(planet.0).remove::<ActivePlanet>();
            }
        }

        mouse_inertia.x = 0.0;
        mouse_inertia.y = 0.0;
    }


    //panning
    if mouse_input.pressed(MouseButton::Right){
        camera_position.translation.x -= mouse_motion.delta.x * zoom.scale;
        camera_position.translation.y += mouse_motion.delta.y * zoom.scale;
    }

    //locking planets

    if keyboard_input.pressed(KeyCode::ControlLeft) && mouse_input.just_pressed(MouseButton::Left){
        if let Some(cursor_pos) = window.cursor_position() {
            let cursor_pos_world = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
            for planet in planets.iter(){
                if planet.5.is_some(){
                    commands.entity(planet.0).remove::<CameraLocked>();
                }

                let dx = cursor_pos_world.x - planet.1.translation.x;
                let dy = cursor_pos_world.y - planet.1.translation.y;
                let r = (planet.1.scale.x * TEXTURE_SIZE as f32 / 2.0) as f32;

                if dx * dx + dy * dy <= r * r{
                    if planet.5.is_some(){
                        commands.entity(planet.0).remove::<CameraLocked>();
                    }
                    else{
                        commands.entity(planet.0).insert(CameraLocked);
                    }
                }
            }
        }
    }



    //zooming

    zoom.scale -= mouse_scroll.delta.y * 40.0;
    if zoom.scale < 40.0{
        zoom.scale = 40.0;
    }
    else if zoom.scale > 1600.0{
        zoom.scale = 1600.0;
    }


    for mut planet in planets.iter_mut(){
        planet.1.scale.x = planet.2.delta;
        planet.1.scale.y = planet.2.delta;

    }


}

fn lock_camera(
    planets: Query<(&Transform, &CameraLocked), (Without<Camera>, Without<PlanetReticle>)>,
    mut reticle: Query<(&mut Transform, &mut Visibility, &PlanetReticle), Without<Camera>>,
    mut camera: Query<(&Camera, &mut Transform,), Without<PlanetReticle>>,
){

    let Ok((_camera, mut camera_position)) = camera.single_mut() else { panic!("no camera!") };
    let Ok((mut reticle_transform, mut visibility, _)) = reticle.single_mut() else { panic!("no reticle!") };

    for planet in planets.iter(){
        camera_position.translation.x = planet.0.translation.x;
        camera_position.translation.y = planet.0.translation.y;
        reticle_transform.scale = planet.0.scale * 0.5;
    }


    if planets.iter().len() > 0{
        reticle_transform.translation = camera_position.translation;
        *visibility = Visibility::Visible;
    }
    else{
        *visibility = Visibility::Hidden;
    }
    
}