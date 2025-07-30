use bevy::{prelude::*, window::PrimaryWindow};

use super::{component::Position, GameState};
pub fn move_mouse_events_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(window) = window.iter().next() {
            if let Some(mouse_position) = window.cursor_position() {
                let (camera, camera_transform) = camera.single().unwrap();
                if let Ok(mouse_position) =
                    camera.viewport_to_world_2d(camera_transform, mouse_position)
                {
                    let row = (mouse_position.y as i32 / 128) as u8;
                    let col = (mouse_position.x as i32 / 128) as u8;
                    if game_state.selected_position.is_none() {
                        game_state.selected_position = Some(Position { row, col });
                    } else {
                        let position = game_state.selected_position.as_mut().unwrap();
                        position.col = col;
                        position.row = row;
                    }
                }
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Right) {
        if let Some(window) = window.iter().next() {
            if let Some(mouse_position) = window.cursor_position() {
                let (camera, camera_transform) = camera.single().unwrap();
                if let Ok(mouse_position) =
                    camera.viewport_to_world_2d(camera_transform, mouse_position)
                {
                    let row = (mouse_position.y as i32 / 128) as u8;
                    let col = (mouse_position.x as i32 / 128) as u8;
                    if game_state.move_position.is_none() {
                        game_state.move_position = Some(Position { row, col })
                    } else {
                        let position = game_state.move_position.as_mut().unwrap();
                        position.col = col;
                        position.row = row;
                    }
                }
            }
        }
    }
}
