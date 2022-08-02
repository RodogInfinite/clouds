use bevy::{prelude::{App, ClearColor, Color}, window::WindowDescriptor, DefaultPlugins, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};

use crate::app::{setup::setup, staging::CustomMaterialPlugin};

use super::cameras::{LookTransformPlugin, controller::orbit::OrbitCameraPlugin};

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
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(setup)
        .run();
}
