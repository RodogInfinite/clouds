use bevy::{prelude::Component, render::render_resource::Buffer};

pub mod app;
pub mod stages;

#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}
