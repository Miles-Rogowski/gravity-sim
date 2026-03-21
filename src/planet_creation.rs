use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::asset::RenderAssetUsages;
use rand::*;

use crate::ui::IsInteractingUI;

use std::path::Path;

use image::GenericImageView;


pub struct PlanetCreationPlugin;

impl Plugin for PlanetCreationPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Update, create_planets_on_click);
    }
}

pub const PLANET_COLORS: [bevy::prelude::LinearRgba; 4] = [
    LinearRgba::rgb(0.19, 0.63, 0.38),
    LinearRgba::rgb(0.16, 0.29, 0.79),
    LinearRgba::rgb(0.70, 0.41, 0.22),
    LinearRgba::rgb(0.33, 0.33, 0.35)
];


pub const MAX_VELOCITY: f32 = 5.0;
pub const MIN_DENSITY: f32 = 0.2;
pub const MAX_DENSITY: f32 = 5.0;

pub const TEXTURE_SIZE: u32 = 400;
pub const SCALE_MULTIPLIER: f32 = 0.1;



#[derive(Component)]
pub struct Forming;

#[derive(Component)]
pub struct Formed;

#[derive(Component)]
pub struct Velocity{
    pub x: f32,
    pub y: f32,
    pub start_x: f32,
    pub start_y: f32
}

#[derive(Component)]
pub struct Mass{
    pub mass: f32,
    pub density: f32,
    pub debris_multiplier: i32
}

#[derive(Component)]
pub struct Scale{
    pub delta: f32
}


#[derive(Component)]
pub struct AbsorbTimer(pub f32);


fn create_planets_on_click(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    mut planets_forming: Query<(Entity, &Forming, &mut Transform, &mut Mass, &mut Scale, &mut Velocity), Without<Camera>>,
    is_interacting_ui: Res<IsInteractingUI>,
    camera: Query<(&Camera, &GlobalTransform, &Transform, &Projection)>,
){
    let mut rng = rand::rng();

    let window = window.single().unwrap();
    let mouse_position = window.cursor_position();

    let Ok((_camera, _camera_transform, camera_position, projection)) = camera.single() else { panic!("no camera!") };
    let Projection::Orthographic(ref zoom) = *projection else { panic!("no projection!") };

    if mouse_input.just_pressed(MouseButton::Left) && planets_forming.iter().len() < 1 && !keyboard_input.pressed(KeyCode::ControlLeft) && !is_interacting_ui.0{
        //create planet

        let x = (mouse_position.unwrap().x - window.width() / 2.0) * zoom.scale + camera_position.translation.x;
        let y = -(mouse_position.unwrap().y - window.height() / 2.0) * zoom.scale + camera_position.translation.y;

        let texture = generate_planet_texture(TEXTURE_SIZE, TEXTURE_SIZE, (TEXTURE_SIZE / 2) as f32, PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())], PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())]);

        let vel_x = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);
        let vel_y = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);

        let dens = rng.random_range(MIN_DENSITY..MAX_DENSITY);

        commands.spawn((
            Forming{},
            Sprite{ image: images.add(texture), ..default() },
            Transform::from_xyz(x, y, 5.0).with_scale(Vec3{ x: 0.002, y: 0.002, z: 1.0 }),
            Velocity{ x: vel_x, y: vel_y, start_x: vel_x, start_y: vel_y },
            Mass{ mass: 0.0, density: dens, debris_multiplier: 1 },
            Scale{ delta: SCALE_MULTIPLIER },
            AbsorbTimer( 0.0 )
        ));
    }
    else if mouse_input.pressed(MouseButton::Left){
        //expand planet

        if let Some((_, _, mut transform, mut mass, mut scale, mut velocity)) = planets_forming.iter_mut().next(){



            scale.delta += SCALE_MULTIPLIER * 2.0;
            

            if mouse_position.is_some(){

                let mouse_x = (mouse_position.unwrap().x - window.width() / 2.0) * zoom.scale + camera_position.translation.x;
                let mouse_y = -(mouse_position.unwrap().y - window.height() / 2.0) * zoom.scale + camera_position.translation.y;

                velocity.x = -(transform.translation.x - mouse_x);
                velocity.y = -(transform.translation.y - mouse_y);

                if velocity.x == 0.0 && velocity.y == 0.0{
                    velocity.x = velocity.start_x;
                    velocity.y = velocity.start_y;
                }

                transform.translation.x = mouse_x;
                transform.translation.y = mouse_y;
            }

            

            mass.mass += mass.density * 2.0 * 40.0;

            //println!("{}", mass.mass);
        }


    }
    else if mouse_input.just_released(MouseButton::Left){
        //allow gravity to act
        if let Some((planet, _, _, _, _, _)) = planets_forming.iter().next(){
            commands.entity(planet).remove::<Forming>();
            commands.entity(planet).insert(Formed);
        }

    }
}

pub fn generate_planet_texture(width: u32, height: u32, radius: f32, color1: LinearRgba, mut color2: LinearRgba) -> Image{

    let mut rng = rand::rng();

    // Modified from - https://stackoverflow.com/q/40518713
    // Posted by BMBM, modified by community. See post 'Timeline' for change history
    // Retrieved 2026-03-19, License - CC BY-SA 3.0

    let image_number = rng.random_range(0..15).to_string();

    let input_path = "assets/".to_owned() + &"noise".to_owned() + &image_number + ".png";

    let img = image::open(&Path::new(&input_path)).unwrap();

    let img_width = img.dimensions().0 as i32;
    let img_height = img.dimensions().1 as i32;

    let raw_pixels = img.to_rgba8().into_raw();
    let mut pixels: Vec<u8> = Vec::new();
    
    let i_height = height as i32;
    let i_width = width as i32;

    let x_offset = rng.random_range(width/2..img_width as u32 - width / 2) as i32;
    let y_offset = rng.random_range(height/2..img_height as u32 - height / 2) as i32;

    if color1 == color2{
        color2.red += rng.random_range(-25..25) as f32 / 255.0;
        color2.green += rng.random_range(-25..25) as f32 / 255.0;
        color2.blue += rng.random_range(-25..25) as f32 / 255.0;
    }

    

    for y in -i_height / 2..i_height / 2{
        for x in -i_width / 2..i_width / 2{
            if (x*x + y*y) as f32 <= radius*radius{
                if raw_pixels[(((y + y_offset) * img_width + (x + x_offset)) * 4) as usize] == 255{
                    pixels.push((color1.red * 255.0) as u8);
                    pixels.push((color1.green * 255.0) as u8);
                    pixels.push((color1.blue * 255.0) as u8);
                    pixels.push(255);
                }
                else{
                    pixels.push((color2.red * 255.0) as u8);
                    pixels.push((color2.green * 255.0) as u8);
                    pixels.push((color2.blue * 255.0) as u8);
                    pixels.push(255);
                }
            }
            else{
                pixels.push(0 as u8);
                pixels.push(0 as u8);
                pixels.push(0 as u8);
                pixels.push(0 as u8);
            }
            
        }
    }

    


    let texture = Image::new(
        Extent3d { width: width, height: height, depth_or_array_layers: 1 },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    return texture;

}