use bevy::{
    color::palettes::basic::*,
    input_focus::{
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
        InputDispatchPlugin, InputFocus,
    },
    picking::hover::Hovered,
    prelude::*,
    ui::{Checked, InteractionDisabled, Pressed},
    ui_widgets::{
        checkbox_self_update, observe,
        popover::{Popover, PopoverAlign, PopoverPlacement, PopoverSide},
        Activate, Button, Checkbox, CoreSliderDragState, MenuAction, MenuButton, MenuEvent,
        MenuItem, MenuPopup, RadioButton, RadioGroup, Slider, SliderRange, SliderThumb,
        SliderValue, TrackClick, UiWidgetsPlugins, ValueChange,
    },
};


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
            slider_value: 50.0,
            slider_click: TrackClick::Snap,
        })
        .add_systems(Startup, setup)
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
const ELEMENT_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);
const ELEMENT_FILL: Color = Color::srgb(0.35, 0.75, 0.35);
const ELEMENT_FILL_DISABLED: Color = Color::srgb(0.501, 0.501, 0.501);


//markers for different ui elements

#[derive(Component)]
struct DemoSlider;

#[derive(Component)]
struct DemoSliderThumb;

#[derive(Component)]
struct DemoMenuAnchor;

#[derive(Component)]
struct DemoMenuItem;


//struct used to keep track of Widget states

#[derive(Resource)]
struct DemoWidgetStates{
    slider_value: f32,
    slider_click: TrackClick,
}

fn update_widget_values( 
    res: Res<DemoWidgetStates>,
    mut sliders: Query<(Entity, &mut Slider), With<DemoSlider>>,
    mut commands: Commands,
){
    if res.is_changed(){
        for (slider_ent, mut slider) in sliders.iter_mut(){
            commands
                .entity(slider_ent)
                .insert(SliderValue(res.slider_value));
            slider.track_click = res.slider_click;
        }
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
){
    //magic thing
    commands.spawn(demo_root(&assets));
}

fn demo_root(
    asset_server: &AssetServer
) -> impl Bundle{
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        TabGroup::default(),
        children![
            (
                slider(0.0, 100.0, 50.0),
                observe(
                    |value_change: On<ValueChange<f32>>,
                    mut widget_states: ResMut<DemoWidgetStates>|{
                        widget_states.slider_value = value_change.value;
                    },
                )
            )
        ]
    )
}


// create slider :O
fn slider(min: f32, max: f32, value: f32) -> impl Bundle{
    (
        Node{
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: percent(30),
            ..default()
        },
        Name::new("Slider"),
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
){
    for (slider_ent, value, range, hovered, drag_state, disabled) in sliders.iter(){
        for child in children.iter_descendants(slider_ent){
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child) && is_thumb{
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
                thumb_bg.0 = thumb_color(disabled, hovered.0 | drag_state.dragging);
            }
        }
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