use bevy::{
    color::palettes::css::*, ecs::name, input_focus::{
        InputDispatchPlugin, tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin}
    }, math::ops, picking::hover::Hovered, prelude::*, sprite::{Anchor, Text2dShadow}, text::{FontSmoothing, LineBreak, TextBounds}, ui::InteractionDisabled, ui_widgets::{
        CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue, TrackClick, UiWidgetsPlugins, ValueChange, observe
    }
};
use std::{collections::HashMap, hash::Hash};


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
        .insert_resource(DemoWidgetStates {
            sliders: HashMap::from([
                (String::from("demo_slider_1"), SliderState{
                    slider_value: 50.0,
                    slider_click: TrackClick::Snap,
                }),
                (String::from("demo_slider_2"), SliderState{
                    slider_value: 50.0,
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
                update_slider_style.after(update_widget_values),
                update_slider_style2.after(update_widget_values),
                toggle_disabled,
            ),
        );
    }
}

const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
const ELEMENT_FILL_DISABLED: Color = Color::srgb(0.501, 0.501, 0.501);


//interactable elements

//markers for different ui elements

#[derive(Component)]
struct DemoSlider;

#[derive(Component)]
struct DemoSliderThumb;

//struct used to keep track of Widget states

struct SliderState{
    slider_value: f32,
    slider_click: TrackClick,
}

//slider data

#[derive(Resource)]
struct DemoWidgetStates{
    sliders: HashMap<String, SliderState>
}

#[derive(Resource)]
pub struct IsInteractingUI(pub bool);



fn update_widget_values( 
    res: Res<DemoWidgetStates>,
    mut sliders: Query<(Entity, &mut Slider, &Name), With<DemoSlider>>,
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

    commands.spawn(demo_root(text_font));
}

fn demo_root(text_font: TextFont) -> impl Bundle{
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
                Text::new(" Demo Text "),
                text_font.clone().with_font_smoothing(FontSmoothing::AntiAliased),
            ),
            (
                slider(0.0, 100.0, 50.0, "demo_slider_1".to_string()),
                observe(
                    |value_change: On<ValueChange<f32>>,
                    mut widget_states: ResMut<DemoWidgetStates>|{
                        let slider_click = widget_states.sliders["demo_slider_1"].slider_click;
                        widget_states.sliders.insert("demo_slider_1".to_string(), SliderState { slider_value: value_change.value, slider_click: slider_click });
                    },
                )
            ),
            (
                slider(0.0, 100.0, 50.0, "demo_slider_2".to_string()),
                observe(
                    |value_change: On<ValueChange<f32>>,
                    mut widget_states: ResMut<DemoWidgetStates>|{
                        let slider_click = widget_states.sliders["demo_slider_2"].slider_click;
                        widget_states.sliders.insert("demo_slider_2".to_string(), SliderState { slider_value: value_change.value, slider_click: slider_click });
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
            width: percent(20),
            ..default()
        },
        Name::new(name),
        Hovered::default(),
        DemoSlider,
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
                    DemoSliderThumb,
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
            With<DemoSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<DemoSliderThumb>), Without<DemoSlider>>,
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
        println!("{}", is_interacting_ui.0);
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
        With<DemoSlider>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut BackgroundColor, Has<DemoSliderThumb>), Without<DemoSlider>>,
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

fn toggle_disabled(
    input: Res<ButtonInput<KeyCode>>,
    mut interaction_query: Query<
        (Entity, Has<InteractionDisabled>),
        Or<(
            With<Slider>,
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