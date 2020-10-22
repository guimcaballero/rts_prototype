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

struct AvailableButtons {
    buttons: Vec<(String, AbilityChangeCallback)>,
}
impl FromResources for AvailableButtons {
    fn from_resources(_resources: &Resources) -> Self {
        AvailableButtons {
            buttons: vec![
                ("Switch control".to_string(), |mut ability, _| {
                    ability.ability = Ability::SwitchCamera;
                }),
                ("Switch back".to_string(), |mut ability, _| {
                    ability.ability = Ability::SwitchBack;
                }),
                ("Add button".to_string(), |_, mut buttons| {
                    buttons.buttons.push(("Nothing".to_string(), |_, _| {
                        println!("nothing");
                    }));
                }),
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

    let entity = commands
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
            for (string, callback) in &available_buttons.buttons {
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
                    .with(AbilityButton(*callback))
                    .with_children(|parent| {
                        parent.spawn(TextComponents {
                            text: Text {
                                value: string.clone(),
                                font: assets.font.clone(),
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                },
                            },
                            ..Default::default()
                        });
                    });
            }
        })
        .current_entity()
        .unwrap();
    displayed_buttons.entities.push(entity);
}

type AbilityChangeCallback = fn(ResMut<CurrentAbility>, ResMut<AvailableButtons>) -> ();

struct AbilityButton(AbilityChangeCallback);
fn button_system(
    ability: ResMut<CurrentAbility>,
    available_buttons: ResMut<AvailableButtons>,
    mut interaction_query: Query<(&mut AbilityButton, Mutated<Interaction>)>,
) {
    for (ability_button, interaction) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                ability_button.0(ability, available_buttons);
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
