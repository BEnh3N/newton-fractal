use crate::screen_to_coordinate_space;

use super::shader::{Root, ShaderParams};
use bevy::prelude::*;

#[derive(Default, Resource, Debug)]
pub struct DragState {
    selected_entity: Option<Entity>,
}

#[derive(Component)]
pub struct Draggable;

pub fn handle_drag(
    mut drag_state: ResMut<DragState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut draggable_objects: Query<(Entity, &mut Transform, &mut Root), With<Draggable>>,
    params: Res<ShaderParams>,
) {
    let window = windows.get_single().unwrap();
    let cursor_position = if let Some(position) = window.cursor_position() {
        (position - Vec2::new(window.width() / 2.0, window.height() / 2.0)) * Vec2::new(1.0, -1.0)
    } else {
        return;
    };

    // Handle mouse press to select an object
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, transform, _) in draggable_objects.iter() {
            let position = transform.translation;
            let half_size = 10.0;

            // Check if cursor is over the object
            if cursor_position.x > position.x - half_size
                && cursor_position.x < position.x + half_size
                && cursor_position.y > position.y - half_size
                && cursor_position.y < position.y + half_size
            {
                drag_state.selected_entity = Some(entity);
                break;
            }
        }
    }

    // Handle mouse release to drop the object
    if mouse_button_input.just_released(MouseButton::Left) {
        drag_state.selected_entity = None;
    }

    // Move the selected object
    if let Some(selected_entity) = drag_state.selected_entity {
        if let Ok((_, mut transform, mut root)) = draggable_objects.get_mut(selected_entity) {
            transform.translation = cursor_position.extend(transform.translation.z);
            root.pos = screen_to_coordinate_space(cursor_position, &window, &params);
        }
    }
}
