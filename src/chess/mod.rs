mod component;
mod entity;
mod resource;
mod system;
mod asset;
mod event;

use bevy::app::Plugin;
use bevy::prelude::*;
use resource::*;
use system::*;
use asset::*;
use event::*;
pub struct ChessPlugin {}

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_asset::<PiecesInfos>()
        .init_asset::<GameSetting>()
        .init_asset_loader::<PiecesInfosLoader>()
        .init_resource::<GameState>()
        .add_systems(Startup, startup_system)
        .add_systems(First, create_pieces_system)
        .add_systems(First, show_board_system)
        .add_systems(First, show_pieces_system)
        .add_systems(Update, move_mouse_events_system)
        .add_systems(Update, select_pieces_system)
        .add_systems(Update, move_pieces_system);
    }
}
