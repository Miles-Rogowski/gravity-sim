use::bevy::prelude::*;
use::bevy::window::PrimaryWindow;
use::rand::*;

pub struct PlanetCreationPlugin;

impl Plugin for PlanetCreationPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Update, create_planets_on_click);
    }
}

pub const PLANET_COLORS: [bevy::prelude::LinearRgba; 2] = [LinearRgba::rgb(0.14, 0.83, 0.81), LinearRgba::rgb(0.4, 0.14, 0.83)];
pub const MAX_VELOCITY: f32 = 1.0;
pub const MIN_DENSITY: f32 = 0.2;
pub const MAX_DENSITY: f32 = 5.0;



#[derive(Component)]
pub struct Forming;

#[derive(Component)]
pub struct Formed;

#[derive(Component)]
pub struct Velocity{
    pub x: f32,
    pub y: f32
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
pub struct Position{
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct AbsorbTimer(pub f32);


fn create_planets_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    mut planets_forming: Query<(Entity, &Forming, &mut Transform, &mut Mass, &mut Scale), Without<Camera>>,
    camera: Query<(&Camera, &GlobalTransform, &Transform, &Projection)>,
){
    let mut rng = rand::rng();

    let window = window.single().unwrap();
    let mouse_position = window.cursor_position();

    let Ok((_camera, _camera_transform, camera_position, projection)) = camera.single() else { panic!("no camera!") };
    let Projection::Orthographic(ref zoom) = *projection else { panic!("no projection!") };

    if mouse_input.just_pressed(MouseButton::Left) && planets_forming.iter().len() < 1{
        //create planet

        let x = (mouse_position.unwrap().x - window.width() / 2.0) * zoom.scale + camera_position.translation.x;
        let y = -(mouse_position.unwrap().y - window.height() / 2.0) * zoom.scale + camera_position.translation.y;

        let vel_x = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);
        let vel_y = rng.random_range(-MAX_VELOCITY..MAX_VELOCITY);
        
        let color = PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())];

        let dens = rng.random_range(MIN_DENSITY..MAX_DENSITY);

        commands.spawn((
            Forming{},
            Mesh2d(meshes.add(Circle::new(1.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(color)))),
            Transform::from_xyz(x, y, 5.0),
            Velocity{ x: vel_x, y: vel_y },
            Mass{ mass: 0.0, density: dens, debris_multiplier: 1 },
            Scale{ delta: 1.0 },
            Position{ x: x, y: y },
            AbsorbTimer( 0.0 )
        ));
    }
    else if mouse_input.pressed(MouseButton::Left){
        //expand planet

        if let Some((_, _, _, mut mass, mut scale)) = planets_forming.iter_mut().next(){



            scale.delta += 1.0;

            mass.mass += mass.density;

            //println!("{}", mass.mass);
        }


    }
    else if mouse_input.just_released(MouseButton::Left){
        //allow gravity to act
        if let Some((planet, _, _, _, _)) = planets_forming.iter().next(){
            commands.entity(planet).remove::<Forming>();
            commands.entity(planet).insert(Formed);
        }

    }
}