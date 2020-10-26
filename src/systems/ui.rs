use crate::systems::ability::{Ability, CurrentAbility};
use bevy::prelude::*;
use bevy_mod_picking::*;

struct UiAssetsResource {
    material: Handle<ColorMaterial>,
    material_none: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromResources for UiAssetsResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut color_materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        UiAssetsResource {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            material: color_materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            material_none: color_materials.add(Color::NONE.into()),
        }
    }
}

pub type ButtonIdentifier = String;
pub type ButtonTuple = (
    String,
    ButtonIdentifier,
    AbilityChangeCallback,
    CallbackData,
);
#[derive(Default, Clone, Copy)]
pub struct CallbackData {
    pub entity: Option<Entity>,
}

pub struct AvailableButtons {
    buttons: Vec<ButtonTuple>,
}

impl AvailableButtons {
    pub fn add_button(&mut self, button: ButtonTuple) -> Result<ButtonIdentifier, &str> {
        let identifier = button.1.clone();

        // Check that there aren't any buttons with that identifier
        for (_, id, _, _) in &self.buttons {
            if *id == identifier {
                return Err("Already used ID");
            }
        }

        self.buttons.push(button);

        Ok(identifier)
    }

    pub fn remove_button(&mut self, identifier: ButtonIdentifier) {
        self.buttons.retain(|(_, id, _, _)| *id != identifier);
    }
}

impl FromResources for AvailableButtons {
    fn from_resources(_resources: &Resources) -> Self {
        AvailableButtons {
            buttons: vec![
                (
                    "Switch control".to_string(),
                    "switch_camera".to_string(),
                    |_, mut ability, _, _| {
                        ability.ability = Ability::SwitchCamera;
                    },
                    CallbackData::default(),
                ),
                (
                    "Switch back".to_string(),
                    "switch_back_camera".to_string(),
                    |_, mut ability, _, _| {
                        ability.ability = Ability::SwitchBack;
                    },
                    CallbackData::default(),
                ),
                (
                    "Add button".to_string(),
                    "add_nothing_button".to_string(),
                    |_, _, mut buttons, _| {
                        let _ = buttons.add_button((
                            "Nothing".to_string(),
                            "nothing_button".to_string(),
                            |_, _, _, _| {
                                println!("nothing");
                            },
                            CallbackData::default(),
                        ));
                    },
                    CallbackData::default(),
                ),
                (
                    "Remove button".to_string(),
                    "remove_nothing_button".to_string(),
                    |_, _, mut buttons, _| {
                        let _ = buttons.remove_button("nothing_button".to_string());
                    },
                    CallbackData::default(),
                ),
            ],
        }
    }
}

#[derive(Default)]
struct DisplayedButtons {
    entities: Vec<Entity>,
}

fn change_displayed_buttons(
    mut commands: Commands,
    assets: Res<UiAssetsResource>,
    available_buttons: ChangedRes<AvailableButtons>,
    mut displayed_buttons: ResMut<DisplayedButtons>,
) {
    for entity in &displayed_buttons.entities {
        commands.despawn(*entity);
    }
    displayed_buttons.entities = Vec::new();

    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(40.), Val::Percent(30.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                display: Display::Flex,
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: assets.material_none.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            for (string, _id, callback, callback_data) in &available_buttons.buttons {
                // Spawn a new button
                parent
                    .spawn(ButtonComponents {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(150.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: Rect::all(Val::Px(0.)),

                            ..Default::default()
                        },
                        material: assets.material.clone(),
                        ..Default::default()
                    })
                    .with(PickingBlocker {})
                    .with(AbilityButton(*callback, *callback_data))
                    .with_children(|parent| {
                        parent
                            .spawn(TextComponents {
                                text: Text {
                                    value: string.clone(),
                                    font: assets.font.clone(),
                                    style: TextStyle {
                                        font_size: 20.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            })
                            .for_current_entity(|entity| displayed_buttons.entities.push(entity));
                    })
                    .for_current_entity(|entity| displayed_buttons.entities.push(entity));
            }
        })
        .for_current_entity(|entity| displayed_buttons.entities.push(entity));
}

pub type AbilityChangeCallback =
    fn(Commands, ResMut<CurrentAbility>, ResMut<AvailableButtons>, CallbackData) -> ();

struct AbilityButton(AbilityChangeCallback, CallbackData);
fn button_system(
    commands: Commands,
    ability: ResMut<CurrentAbility>,
    available_buttons: ResMut<AvailableButtons>,
    mut interaction_query: Query<(&mut AbilityButton, Mutated<Interaction>)>,
) {
    for (ability_button, interaction) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                ability_button.0(commands, ability, available_buttons, ability_button.1);
                return;
            }
            _ => {}
        }
    }
}

struct PickingBlocker;
fn block_picking_under_blockers(
    mut pick_state: ResMut<PickState>,
    mut interaction_query: Query<(&Button, &Interaction, &PickingBlocker)>,
) {
    let mut some_is_hovered = false;
    for (_button, interaction, _) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked | Interaction::Hovered => {
                some_is_hovered = true;
            }
            Interaction::None => {}
        }
    }
    pick_state.enabled = !some_is_hovered;
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<UiAssetsResource>()
            .init_resource::<AvailableButtons>()
            .init_resource::<DisplayedButtons>()
            .add_system(block_picking_under_blockers.system())
            .add_system(button_system.system())
            .add_system(change_displayed_buttons.system());
    }
}
