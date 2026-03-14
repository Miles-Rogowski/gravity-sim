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





#[derive(Component)]
struct Forming;

fn create_planets_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    mut planets_forming: Query<(Entity, &Forming, &mut Transform)>
){
    let mut rng = rand::rng();

    let window = window.single().unwrap();
    let mouse_position = window.cursor_position();


    if mouse_input.just_pressed(MouseButton::Left){
        //create planet
        let x = mouse_position.unwrap().x - window.width() / 2.0;
        let y = -(mouse_position.unwrap().y - window.height() / 2.0);
        
        let color = PLANET_COLORS[rng.random_range(0..PLANET_COLORS.len())];

        commands.spawn((
            Forming{},
            Mesh2d(meshes.add(Circle::new(1.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(color)))),
            Transform::from_xyz(x, y, 5.0),
        ));
    }
    else if mouse_input.pressed(MouseButton::Left){
        //expand planet
        if mouse_position.is_some(){
            println!("{}, {}", mouse_position.unwrap().x, mouse_position.unwrap().y);
        }

        if let Some((_, _, mut transform)) = planets_forming.iter_mut().next(){
            transform.scale.x += 2.5;
            transform.scale.y += 2.5;
        }


    }
    else if mouse_input.just_released(MouseButton::Left){
        //allow gravity to act
        if let Some((planet, _, _)) = planets_forming.iter().next(){
            commands.entity(planet).remove::<Forming>();
        }

    }
}