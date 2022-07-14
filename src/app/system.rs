use bevy::{prelude::App, window::WindowDescriptor, DefaultPlugins};

use crate::app::{setup::setup, staging::CustomMaterialPlugin};

pub fn run() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomMaterialPlugin)
        .add_startup_system(setup)
        .run();
}
