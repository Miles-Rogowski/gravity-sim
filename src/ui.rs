use bevy::{
    color::palettes::basic::*,
    input_focus::{
        InputDispatchPlugin, tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin}
    }, picking::hover::Hovered, prelude::*, text::FontSmoothing, ui::{InteractionDisabled, Pressed}, ui_widgets::{
        Activate, Button, CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue, TrackClick, UiWidgetsPlugins, ValueChange, observe
    }
};
use std::collections::HashMap;


pub struct UIPlugin;


//used lots of code from https://bevy.org/examples/ui-user-interface/standard-widgets/
//the bevy website has lots of great examples



impl Plugin for UIPlugin {
    fn build(&self, app: &mut App){
        app
        .add_plugins((
            UiWidgetsPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin,
        ))
        .insert_resource(SliderWidgetStates {
            sliders: HashMap::from([
                (String::from("Planet Creation Speed"), SliderState{
                    slider_value: 1.0,
                    slider_click: TrackClick::Snap,
                }),
                (String::from("Throw Strength"), SliderState{
                    slider_value: 1.0,
                    slider_click: TrackClick::Snap,
                }),
                (String::from("Gravity Multiplier"), SliderState{
                    slider_value: 1.0,
                    slider_click: TrackClick::Snap,
                }),
            ])
        })
        .insert_resource(IsInteractingUI(false))
        .add_systems(Startup, (
            spawn_ui,
            //create_text,
        ))
        .add_systems(
            Update,
            (
                update_widget_values,
                update_button_style,
                update_button_style2,
                update_slider_style.after(update_widget_values),
                update_slider_style2.after(update_widget_values),
                toggle_disabled,
            ),
        );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);

const ELEMENT_FILL_DISABLED: Color = Color::srgb(0.501, 0.501, 0.501);


//interactable elements

//markers for different ui elements

#[derive(Component)]
struct UIButton;

#[derive(Component)]
struct UISlider;

#[derive(Component)]
struct UISliderThumb;

//struct used to keep track of Widget states

pub struct SliderState{
    pub slider_value: f32,
    slider_click: TrackClick,
}

//slider data

#[derive(Resource)]
pub struct SliderWidgetStates{
    pub sliders: HashMap<String, SliderState>
}

#[derive(Resource)]
pub struct IsInteractingUI(pub bool);



fn update_widget_values( 
    res: Res<SliderWidgetStates>,
    mut sliders: Query<(Entity, &mut Slider, &Name), With<UISlider>>,
    mut commands: Commands,
){
    if res.is_changed(){
        for (slider_ent, mut slider, name) in sliders.iter_mut(){
            commands
                .entity(slider_ent)
                .insert(SliderValue(res.sliders[name.as_str()].slider_value));
            slider.track_click = res.sliders[name.as_str()].slider_click;
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let font = asset_server.load("FiraSans-Bold.ttf");
    let text_font = TextFont{
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    commands.spawn(demo_root(text_font, &asset_server));
}

fn demo_root(text_font: TextFont, asset_server: &AssetServer) -> impl Bundle{
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: px(25).into(),
            row_gap: px(10),
            ..default()
        },
        TabGroup::default(),
        children![
            (
                button(asset_server),
                observe(|_activate: On<Activate>, mut widget_states: ResMut<SliderWidgetStates>,|{
                    println!("button clicked");
                    let keys: Vec<String> = widget_states.sliders.keys().cloned().collect();
                    for key in keys{
                        let slider_click = widget_states.sliders[&key].slider_click;
                        widget_states.sliders.insert(key.to_string(), SliderState { slider_value: 1.0, slider_click: slider_click });
                    }
                }),
            ),
            (
                Text::new(" Planet Creation Speed "),
                text_font.clone().with_font_smoothing(FontSmoothing::AntiAliased),
            ),
            (
                slider(0.1, 10.0, 1.0, "Planet Creation Speed".to_string()),
                observe(
                    |value_change: On<ValueChange<f32>>,
                    mut widget_states: ResMut<SliderWidgetStates>|{
                        let slider_click = widget_states.sliders["Planet Creation Speed"].slider_click;
                        widget_states.sliders.insert("Planet Creation Speed".to_string(), SliderState { slider_value: value_change.value, slider_click: slider_click });
                    },
                )
            ),
            (
                Text::new(" Throw Strength "),
                text_font.clone().with_font_smoothing(FontSmoothing::AntiAliased),
            ),
            (
                slider(0.1, 10.0, 1.0, "Throw Strength".to_string()),
                observe(
                    |value_change: On<ValueChange<f32>>,
                    mut widget_states: ResMut<SliderWidgetStates>|{
                        let slider_click = widget_states.sliders["Throw Strength"].slider_click;
                        widget_states.sliders.insert("Throw Strength".to_string(), SliderState { slider_value: value_change.value, slider_click: slider_click });
                    },
                )
            ),
            (
                Text::new(" Gravity Multiplier "),
                text_font.clone().with_font_smoothing(FontSmoothing::AntiAliased),
            ),
            (
                slider(-10.0, 10.0, 1.0, "Gravity Multiplier".to_string()),
                observe(
                    |value_change: On<ValueChange<f32>>,
                    mut widget_states: ResMut<SliderWidgetStates>|{
                        let slider_click = widget_states.sliders["Gravity Multiplier"].slider_click;
                        widget_states.sliders.insert("Gravity Multiplier".to_string(), SliderState { slider_value: value_change.value, slider_click: slider_click });
                    },
                )
            )
            
            
        ]
    )
}


// create slider :O
fn slider(min: f32, max: f32, value: f32, name: String) -> impl Bundle{
    (
        Node{
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: px(250),
            ..default()
        },
        Name::new(name),
        Hovered::default(),
        UISlider,
        Slider{
            track_click: TrackClick::Snap,
        },
        SliderValue(value),
        SliderRange::new(min, max),
        TabIndex(0),
        Children::spawn((
            Spawn((
                Node{
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK), // border color
            )),
            
            // Invisible track to allow absolute placement of thumb entity. This is narrower than
            // the actual slider, which allows us to position the thumb entity using simple
            // percentages, without having to measure the actual width of the slider thumb.
            
            Spawn((
                Node{
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    // Track is short by 12px to accommodate the thumb.
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    //thumb
                    UISliderThumb,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}

fn update_slider_style(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<SliderRange>,
                Changed<Hovered>,
                Changed<CoreSliderDragState>,
                Added<InteractionDisabled>,
            )>,
            With<UISlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<UISliderThumb>), Without<UISlider>>,
    mut is_interacting_ui: ResMut<IsInteractingUI>,
){
    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter(){
        for child in children.iter_descendants(slider_ent){
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child) && is_thumb{
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
                thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
            }
        }
        is_interacting_ui.0 = hovered.0;
    }
}

//apple

fn update_slider_style2(
    sliders: Query<
        (
            Entity,
            &Hovered,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        With<UISlider>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut BackgroundColor, Has<UISliderThumb>), Without<UISlider>>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
){
    removed_disabled.read().for_each(|entity| {
        if let Ok((slider_ent, hovered, drag_state, disabled)) = sliders.get(entity){
            for child in children.iter_descendants(slider_ent){
                if let Ok((mut thumb_bg, is_thumb)) = thumbs.get_mut(child) && is_thumb{
                    thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
                }
            }
        }
    });
}

fn thumb_color(disabled: bool, hovered: bool) -> Color{
    match (disabled, hovered){
        (true, _) => ELEMENT_FILL_DISABLED,

        (false, true) => SLIDER_THUMB.lighter(0.3),

        _ => SLIDER_THUMB,
    }
}


//buttons :O
//help im tired :P

fn button(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::MAX,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        UIButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new("Reset To Defaults"),
            TextFont {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

fn update_button_style(
    mut buttons: Query<
        (
            Has<Pressed>,
            &Hovered,
            Has<InteractionDisabled>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Or<(
                Changed<Pressed>,
                Changed<Hovered>,
                Added<InteractionDisabled>,
            )>,
            With<UIButton>,
        ),
    >,
    mut is_interacting_ui: ResMut<IsInteractingUI>,
) {
    for (pressed, hovered, disabled, mut color, mut border_color, _) in &mut buttons {
        set_button_style(
            disabled,
            hovered.get(),
            pressed,
            &mut color,
            &mut border_color,
        );
        is_interacting_ui.0 = hovered.0;
    }
}


fn update_button_style2(
    mut buttons: Query<
        (
            Has<Pressed>,
            &Hovered,
            Has<InteractionDisabled>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<UIButton>,
    >,
    mut removed_depressed: RemovedComponents<Pressed>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
) {
    removed_depressed
        .read()
        .chain(removed_disabled.read())
        .for_each(|entity| {
            if let Ok((pressed, hovered, disabled, mut color, mut border_color, _)) =
                buttons.get_mut(entity)
            {
                set_button_style(
                    disabled,
                    hovered.get(),
                    pressed,
                    &mut color,
                    &mut border_color,
                );
            }
        });
}


fn set_button_style(
    disabled: bool,
    hovered: bool,
    pressed: bool,
    color: &mut BackgroundColor,
    border_color: &mut BorderColor,
){
    match (disabled, hovered, pressed) {
        // Disabled button
        (true, _, _) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(GRAY);
        }

        // Pressed and hovered button
        (false, true, true) => {
            *color = PRESSED_BUTTON.into();
            border_color.set_all(RED);
        }

        // Hovered, unpressed button
        (false, true, false) => {
            *color = HOVERED_BUTTON.into();
            border_color.set_all(WHITE);
        }

        // Unhovered button (either pressed or not).
        (false, false, _) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(BLACK);
        }
    }

}



fn toggle_disabled(
    input: Res<ButtonInput<KeyCode>>,
    mut interaction_query: Query<
        (Entity, Has<InteractionDisabled>),
        Or<(
            With<Slider>,
            With<Button>,
        )>,
    >,
    mut commands: Commands,
){
    if input.just_pressed(KeyCode::KeyD){
        for (entity, disabled) in &mut interaction_query{
            if disabled{
                info!("widget enabled");
                commands.entity(entity).remove::<InteractionDisabled>();
            }
            else{
                info!("Widget disabled");
                commands.entity(entity).insert(InteractionDisabled);
            }
        }
    }
}