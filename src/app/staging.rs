use bevy::{
    core_pipeline::core_3d::Opaque3d,
    prelude::{App, Plugin},
    render::{
        extract_component::ExtractComponentPlugin, render_phase::AddRenderCommand,
        render_resource::SpecializedMeshPipelines, RenderApp, RenderStage,
    },
};

use crate::stages::{
    extract::InstanceMaterialData,
    prepare::prepare_instance_buffers,
    queueing::{draw::DrawCustom, pipeline::CustomPipeline, queue::queue_custom},
};

pub struct PointCloudPlugin;

impl Plugin for PointCloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Opaque3d, DrawCustom>()
            .init_resource::<CustomPipeline>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_system_to_stage(RenderStage::Queue, queue_custom)
            .add_system_to_stage(RenderStage::Prepare, prepare_instance_buffers);
    }
}
