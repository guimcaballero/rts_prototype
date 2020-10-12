use bevy::prelude::*;

fn create_ui(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Button".to_string(),
                    font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        });
}

fn button_system(
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &Children)>,
    text_query: Query<&mut Text>,
) {
    for (_button, interaction, children) in &mut interaction_query.iter() {
        let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
            }
            Interaction::None => {
                text.value = "Button".to_string();
            }
        }
    }
}

#[derive(Default)]
pub struct HoveringUI(pub bool);
struct PickingBlocker;
fn block_picking_under_blockers(
    mut hovering_ui: ResMut<HoveringUI>,
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
    hovering_ui.0 = some_is_hovered;
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<HoveringUI>()
            .add_system(block_picking_under_blockers.system())
            .add_startup_system(create_ui.system())
            .add_system(button_system.system());
    }
}
