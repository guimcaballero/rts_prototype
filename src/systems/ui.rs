use crate::systems::ability::{Ability, CurrentAbility};
use bevy::prelude::*;
use bevy_mod_picking::*;

fn create_ui(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: color_materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            ..Default::default()
        })
        .with(PickingBlocker {})
        .with(AbilityButton(box |mut ability| {
            ability.ability = Ability::SwitchCamera;
            println!("changed ability to Switch Camera");
        }))
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Switch control".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        });
}

type AbilityChangeCallback = Box<dyn FnMut(ResMut<CurrentAbility>) -> () + Send + Sync>;

struct AbilityButton(AbilityChangeCallback);
fn button_system(
    ability: ResMut<CurrentAbility>,
    mut interaction_query: Query<(&mut AbilityButton, Mutated<Interaction>)>,
) {
    for (mut ability_button, interaction) in &mut interaction_query.iter() {
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
        app.add_system(block_picking_under_blockers.system())
            .add_startup_system(create_ui.system())
            .add_system(button_system.system());
    }
}
