use crate::systems::ability::{Ability, CurrentAbility};
use bevy::prelude::*;
use bevy_mod_picking::*;

struct UiAssetsResource {
    material: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromResources for UiAssetsResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut color_materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        UiAssetsResource {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            material: color_materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
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
                ("Switch control".to_string(), |mut ability| {
                    ability.ability = Ability::SwitchCamera;
                    println!("changed ability to Switch Camera");
                }),
                ("Switch control".to_string(), |mut ability| {
                    ability.ability = Ability::SwitchCamera;
                    println!("changed ability to Switch Camera");
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

    let mut i = 0.;
    for (string, callback) in &available_buttons.buttons {
        // Spawn a new button and add the entity to displayed
        let entity = commands
            .spawn(ButtonComponents {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(10.0),
                        bottom: Val::Px(i * 75. + 10.0),
                        ..Default::default()
                    },
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
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    },
                    ..Default::default()
                });
            })
            .current_entity()
            .unwrap();

        displayed_buttons.entities.push(entity);
        i += 1.;
    }
}

type AbilityChangeCallback = fn(ResMut<CurrentAbility>) -> ();

struct AbilityButton(AbilityChangeCallback);
fn button_system(
    ability: ResMut<CurrentAbility>,
    mut interaction_query: Query<(&mut AbilityButton, Mutated<Interaction>)>,
) {
    for (ability_button, interaction) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                ability_button.0(ability);
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
            .add_startup_system(change_displayed_buttons.system())
            .add_system(button_system.system());
    }
}
