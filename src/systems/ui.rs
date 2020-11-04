use crate::systems::{
    ability::{Ability, CurrentAbility},
    selection_circle::*,
};
use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct UiAssetsResource {
    material: Handle<ColorMaterial>,
    material_none: Handle<ColorMaterial>,
    pub font: Handle<Font>,
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
    pub associated_circle: Option<Entity>,
}

pub struct AvailableButtons {
    buttons: Vec<ButtonTuple>,
    dirty: bool,
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
        self.dirty = true;

        Ok(identifier)
    }

    pub fn remove_button(&mut self, identifier: ButtonIdentifier) {
        let old_len = self.buttons.len();

        // Remove buttons with identifier
        self.buttons.retain(|(_, id, _, _)| *id != identifier);

        let new_len = self.buttons.len();

        if old_len != new_len {
            self.dirty = true;
        }
    }
}

impl FromResources for AvailableButtons {
    fn from_resources(_resources: &Resources) -> Self {
        AvailableButtons {
            dirty: true, // Start as dirty
            buttons: vec![
                (
                    "Switch Camera".to_string(),
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
    mut available_buttons: ResMut<AvailableButtons>,
    mut displayed_buttons: ResMut<DisplayedButtons>,
) {
    if !available_buttons.dirty {
        return;
    }
    available_buttons.dirty = false;

    for entity in &displayed_buttons.entities {
        commands.despawn(*entity);
    }
    displayed_buttons.entities = Vec::new();

    commands
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
                    .with(AbilityButton {
                        callback: *callback,
                        data: *callback_data,
                    })
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
    fn(Commands, ResMut<CurrentAbility>, ResMut<AvailableButtons>, CallbackData);

struct AbilityButton {
    callback: AbilityChangeCallback,
    data: CallbackData,
}
fn button_system(
    commands: Commands,
    ability: ResMut<CurrentAbility>,
    available_buttons: ResMut<AvailableButtons>,
    mut interaction_query: Query<(&mut AbilityButton, Mutated<Interaction>)>,
    mut circle_query: Query<&mut SelectionCircle>,
) {
    for (ability_button, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            (ability_button.callback)(commands, ability, available_buttons, ability_button.data);
            return;
        }

        if let Some(associated_entity) = ability_button.data.associated_circle {
            if let Ok(mut circle) =
                circle_query.get_component_mut::<SelectionCircle>(associated_entity)
            {
                circle.unit_highlighted = *interaction == Interaction::Hovered;
            }
        }
    }
}

struct PickingBlocker;
fn block_picking_under_blockers(
    mut pick_state: ResMut<PickState>,
    mut interaction_query: Query<(&Button, &Interaction, &PickingBlocker)>,
) {
    let mut some_is_hovered = false;
    for (_button, interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked | Interaction::Hovered => {
                some_is_hovered = true;
            }
            Interaction::None => {}
        }
    }
    pick_state.enabled = !some_is_hovered;
}

// Ability display UI
struct AbilityText;
fn init_ability_text(mut commands: Commands, assets: Res<UiAssetsResource>) {
    commands
        // root node
        .spawn(NodeComponents {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: assets.material_none.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextComponents {
                    text: Text {
                        value: "Ability: None".to_string(),
                        font: assets.font.clone(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                })
                .with(AbilityText);
        });
}

fn ability_text_update(ability: Res<CurrentAbility>, mut query: Query<(&mut Text, &AbilityText)>) {
    for (mut text, _tag) in query.iter_mut() {
        text.value = format!("Ability: {}", ability.ability);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<UiAssetsResource>()
            .init_resource::<AvailableButtons>()
            .init_resource::<DisplayedButtons>()
            .add_startup_system(init_ability_text.system())
            .add_system(ability_text_update.system())
            .add_system(block_picking_under_blockers.system())
            .add_system(button_system.system())
            .add_system(change_displayed_buttons.system());
    }
}
