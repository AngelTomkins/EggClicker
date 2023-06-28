use bevy::prelude::*;

use app::ApplicationPlugin;
use ui::UIPlugin;

mod app;
mod config;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(ApplicationPlugin)
        .add_plugin(UIPlugin)
        .run();
}
