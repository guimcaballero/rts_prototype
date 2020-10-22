use crate::systems::{
    ability::*,
    selection::Selectable,
    unit::{TargetPosition, Unit},
};
use bevy::{prelude::*, render::camera::Camera};
use bevy_mod_picking::*;

#[derive(Default)]
pub struct CanHaveCamera;

#[derive(Default)]
pub struct CameraFollow {
    pub entity: Option<Entity>,
    pub previous_entity: Option<Entity>,
    pub translation_offset: Vec3,
    pub rotation_offset: Quat,
}

/// Sets the camera position to whatever the current object that has it
fn update_camera_position(
    mut camera_query: Query<(&Camera, &CameraFollow, &mut Transform)>,
    has_camera_query: Query<(&CanHaveCamera, &Transform)>,
) {
    for (_, camera_follow, mut transform) in &mut camera_query.iter() {
        if let Some(following) = camera_follow.entity {
            if let Ok(parent_transform) = has_camera_query.get::<Transform>(following) {
                let new_translation =
                    parent_transform.translation + camera_follow.translation_offset;
                let new_rotation = parent_transform.rotation * camera_follow.rotation_offset;

                *transform = Transform::from_matrix(Mat4::from_rotation_translation(
                    new_rotation,
                    new_translation,
                ));
            }
        }
    }
}

fn reset_unit_target_if_it_has_camera(
    mut camera_query: Query<&CameraFollow>,
    mut query: Query<(
        Entity,
        &CanHaveCamera,
        &mut TargetPosition,
        &mut Selectable,
        &mut Draw,
    )>,
) {
    for camera_follow in &mut camera_query.iter() {
        for (entity, _can_have_camera, mut target, mut selectable, mut draw) in &mut query.iter() {
            // TODO This will act weird if there is more than one camera
            if Some(entity) == camera_follow.entity {
                target.pos = None;
                selectable.selected = false;
                draw.is_visible = false;
            } else {
                draw.is_visible = true;
            }
        }
    }
}

fn switch_camera_to_entity(
    mut ability: ResMut<CurrentAbility>,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut CameraFollow>,
    query: Query<(&CanHaveCamera, &Unit)>,
) {
    if ability.ability != Ability::SwitchCamera {
        return;
    }

    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the entity selected
    if let Some(top_pick) = pick_state.top(PickGroup::default()) {
        let entity = top_pick.entity();

        // Check if it's in the CanHaveCamera query
        if let Ok(_) = query.get::<CanHaveCamera>(entity) {
            for mut camera_follow in &mut camera_query.iter() {
                camera_follow.previous_entity = camera_follow.entity;
                camera_follow.entity = Some(entity);
                println!("Changing entity in camera");
            }
        }
    }

    ability.ability = Ability::Select;
}

/// Switches the camera to the previous entity
fn switch_camera_back(
    mut ability: ResMut<CurrentAbility>,
    mut camera_query: Query<&mut CameraFollow>,
    can_have_camera_query: Query<(&CanHaveCamera, &Unit, Entity)>,
) {
    if ability.ability != Ability::SwitchBack {
        return;
    }

    for mut camera_follow in &mut camera_query.iter() {
        if let Some(prev) = camera_follow.previous_entity {
            // Check that the unit is alive
            if can_have_camera_query.get::<CanHaveCamera>(prev).is_ok() {
                camera_follow.previous_entity = camera_follow.entity;
                camera_follow.entity = Some(prev);
            }
        }
    }

    ability.ability = Ability::Select;
}

/// Switches the camera to the previous or a random entity if the current one dies
fn switch_after_current_unit_dies(
    mut camera_query: Query<&mut CameraFollow>,
    mut can_have_camera_query: Query<(&CanHaveCamera, &Unit, Entity)>,
) {
    for mut camera_follow in &mut camera_query.iter() {
        if let Some(following) = camera_follow.entity {
            // If the unit is not in the query, it has died, so we need to change it
            if can_have_camera_query
                .get::<CanHaveCamera>(following)
                .is_err()
            {
                // Check if prev_entity is valid, and go to that one if it is
                if let Some(prev) = camera_follow.previous_entity {
                    if can_have_camera_query.get::<CanHaveCamera>(prev).is_ok() {
                        camera_follow.entity = camera_follow.previous_entity;
                        continue; // Go to next camera
                    }
                }

                // Go to a random unit if not
                for (_, _, entity) in &mut can_have_camera_query.iter() {
                    camera_follow.entity = Some(entity);
                }
            }
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_camera_position.system())
            .add_system(switch_camera_to_entity.system())
            .add_system(reset_unit_target_if_it_has_camera.system())
            .add_system(switch_after_current_unit_dies.system())
            .add_system(switch_camera_back.system());
    }
}
