//! Learning rust and bevy from create a chess game
//!
//! - [Why this ?]
//!
//! [Why this ?]: https://deadpoetspoon.github.io/a-poor-imitation/blog/chess/one/
mod chess;

use bevy::{prelude::*, window::WindowMode};
use chess::ChessPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess Game".to_string(),
                        name: Some("Chess Game".to_string()),
                        mode: WindowMode::Fullscreen(
                            MonitorSelection::Primary,
                            VideoModeSelection::Current,
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(ChessPlugin {})
        .run();
}
