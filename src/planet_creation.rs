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

const PLANET_COLORS: [bevy::prelude::LinearRgba; 2] = [LinearRgba::rgb(0.14, 0.83, 0.81), LinearRgba::rgb(0.4, 0.14, 0.83)];
const MAX_VELOCITY: f32 = 1.0;
const MIN_DENSITY: f32 = 0.2;
const MAX_DENSITY: f32 = 5.0;



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
pub struct AbsorbTimer(pub f32);


fn create_planets_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    mut planets_forming: Query<(Entity, &Forming, &mut Transform, &mut Mass)>
){
    let mut rng = rand::rng();

    let window = window.single().unwrap();
    let mouse_position = window.cursor_position();


    if mouse_input.just_pressed(MouseButton::Left) && planets_forming.iter().len() < 1{
        //create planet
        let x = mouse_position.unwrap().x - window.width() / 2.0;
        let y = -(mouse_position.unwrap().y - window.height() / 2.0);

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
            AbsorbTimer( 0.0 )
        ));
    }
    else if mouse_input.pressed(MouseButton::Left){
        //expand planet

        if let Some((_, _, mut transform, mut mass)) = planets_forming.iter_mut().next(){
            transform.scale.x += 1.0;
            transform.scale.y += 1.0;

            mass.mass += mass.density;

            //println!("{}", mass.mass);
        }


    }
    else if mouse_input.just_released(MouseButton::Left){
        //allow gravity to act
        if let Some((planet, _, _, _)) = planets_forming.iter().next(){
            commands.entity(planet).remove::<Forming>();
            commands.entity(planet).insert(Formed);
        }

    }
}