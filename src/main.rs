

mod chess;


use bevy::prelude::*;
use chess::ChessPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()))
        .add_plugins(ChessPlugin{})
        .run();
}
