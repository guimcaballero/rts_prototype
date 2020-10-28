use crate::helpers::shapes::*;
use crate::systems::selection::*;
use bevy::prelude::*;
use bevy_contrib_colors::*;
use bevy_mod_picking::*;

#[derive(Default)]
pub struct SelectionCircle {
    pub unit_highlighted: bool, // Used to highlight the unit, e.g. when hovering a button
    pub unit_hovered: bool,
    pub unit_selected: bool,
}
impl SelectionCircle {
    pub fn visible(&self) -> bool {
        self.unit_hovered || self.unit_highlighted || self.unit_selected
    }
}

fn move_circle_for_selected_units(
    mut query: Query<(&Selectable, &Transform)>,
    mut circle_query: Query<(&SelectionCircle, &mut Transform)>,
) {
    for (selectable, transform) in &mut query.iter() {
        if let Ok(mut circle) = circle_query.entity(selectable.circle) {
            if let Some((circle, mut circle_transform)) = circle.get() {
                if circle.visible() {
                    let translation = transform.translation;
                    circle_transform.translation = Vec3::new(translation.x(), 0.1, translation.z());
                }
            }
        }
    }
}

fn set_unit_hovered_for_circles(
    pick_state: Res<PickState>,
    mut query: Query<(&Selectable, Entity)>,
    circle_query: Query<&mut SelectionCircle>,
) {
    for (selectable, entity) in &mut query.iter() {
        if let Ok(mut circle) = circle_query.get_mut::<SelectionCircle>(selectable.circle) {
            if let Some(top_pick) = pick_state.top(PickGroup::default()) {
                let top_entity = top_pick.entity();

                if entity == top_entity {
                    circle.unit_hovered = true;
                } else {
                    circle.unit_hovered = false;
                }
            } else {
                circle.unit_hovered = false;
            }
        }
    }
}

fn set_unit_selected_for_circles(
    mut query: Query<Changed<Selectable>>,
    circle_query: Query<&mut SelectionCircle>,
) {
    for selectable in &mut query.iter() {
        if let Ok(mut circle) = circle_query.get_mut::<SelectionCircle>(selectable.circle) {
            circle.unit_selected = selectable.selected;
        }
    }
}

fn change_circle_color(
    resource: Res<SelectionCircleMaterial>,
    mut query: Query<(&SelectionCircle, &mut Draw, &mut Handle<ColorMaterial>)>,
) {
    for (circle, mut draw, mut material) in &mut query.iter() {
        *material = if circle.unit_highlighted {
            resource.highlighted_material.clone()
        } else if circle.unit_hovered {
            resource.hover_material.clone()
        } else {
            resource.selected_material.clone()
        };

        draw.is_visible = circle.visible();
    }
}

pub struct SelectionCircleMaterial {
    pub circle_mesh: Handle<Mesh>,
    pub circle_material: Handle<ColorMaterial>,
    pub selected_material: Handle<ColorMaterial>,
    pub hover_material: Handle<ColorMaterial>,
    pub highlighted_material: Handle<ColorMaterial>,
}
impl FromResources for SelectionCircleMaterial {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        SelectionCircleMaterial {
            circle_mesh: meshes.add(circle_mesh()),
            circle_material: materials.add(Tailwind::BLUE500.into()),
            selected_material: materials.add(Tailwind::BLUE500.into()),
            hover_material: materials.add(Tailwind::BLUE300.into()),
            highlighted_material: materials.add(Tailwind::YELLOW300.into()),
        }
    }
}

pub struct SelectionCirclePlugin;
impl Plugin for SelectionCirclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectionCircleMaterial>()
            .add_system(change_circle_color.system())
            .add_system(set_unit_hovered_for_circles.system())
            .add_system(set_unit_selected_for_circles.system())
            .add_system(move_circle_for_selected_units.system());
    }
}
