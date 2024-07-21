use super::{
    component::{ChessColorKind, Position},
    GameSetting, PiecesInfos,
};
use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct GameState {
    pub selected_position: Option<Position>,
    pub move_position: Option<Position>,
    pub current_turn: ChessColorKind,
    pub game_setting_handle: Handle<GameSetting>,
    pub game_setting_has_load: bool,
    pub pieces_infos_handle: Handle<PiecesInfos>,
    pub pieces_infos_has_load: bool,
}
