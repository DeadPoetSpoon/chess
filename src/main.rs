//! Learning rust and bevy from create a chess game
//!
//! - [Why this ?]
//!
//! [Why this ?]: https://deadpoetspoon.github.io/a-poor-imitation/blog/lesson-one/
mod chess;

use bevy::prelude::*;
use chess::ChessPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin::default()))
        .add_plugins(ChessPlugin {})
        .run();
}
