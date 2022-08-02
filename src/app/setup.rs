use bevy::prelude::*;

use crate::app::cameras::controller::orbit::{OrbitCameraBundle, OrbitCameraController};
use crate::stages::extract::{InstanceData, InstanceMaterialData};
use rand::Rng;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut rng = rand::thread_rng();
    let n = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(400) as f32;
    let hexagon = shape::Icosphere{
        radius: 0.5,
        subdivisions: 2 // Stress test at some point
    };
    info!("Generating {} Icospheres", n*n); 
    commands.
        spawn()
        .insert_bundle(
            (meshes.add(
                Mesh::from(
                        hexagon
                        )
                    ),
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::default(),
                InstanceMaterialData(
                    (1..=n as i32)
                        .flat_map(|x| (1..=n as i32 )
                        .map(move |y| (x as f32 / n, y as f32 / n)))
                        .map(|(x, y)| InstanceData {
                            position: Vec3::new(x * n - n/2.0, y * n  - n/2.0, rng.gen_range(-50.0..=1000.0)),
                            scale: 1.0,
                            color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
                        })
                        .collect(),
                ),
                Visibility::default(),
                ComputedVisibility::default(),
                // NOTE: Frustum culling is done based on the Aabb (axis aligned bounding box) of the Mesh and the GlobalTransform.
                // As the quad is at the origin, if its Aabb moves outside the view frustum, all the
                // instanced quads will be culled.
                // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
                // instancing, and that is not taken into account with the built-in frustum culling.
                // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
                // component to avoid incorrect culling.
                //NoFrustumCulling,
            )
            
        );
    info!("Generated {} Icospheres", n*n); 
    // camera
    commands
    .spawn_bundle(Camera3dBundle::default())
    .insert_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(-2.0, 5.0, 5.0),
        Vec3::new(0., 0., 0.),
    ));
}