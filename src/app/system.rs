use bevy::{prelude::{App, ClearColor, Color}, window::WindowDescriptor, DefaultPlugins};

use crate::app::{setup::setup, staging::CustomMaterialPlugin};

pub fn run() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Background Color
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomMaterialPlugin)
        .add_startup_system(setup)
        .run();
}
