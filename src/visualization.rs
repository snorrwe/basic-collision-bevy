use bevy::asset::load_internal_asset;
use bevy::color::LinearRgba;
use bevy::prelude::*;
use bevy::render::render_resource::{PolygonMode, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::{reflect::TypePath, render::render_resource::AsBindGroup};

use crate::{AABBCollision, AABB};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AABBMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(0xd4e23a038f33f9785249ce29029293c1);

impl Material2d for AABBMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Visualize;

#[derive(Resource)]
struct Store {
    green_material: Handle<AABBMaterial>,
    red_material: Handle<AABBMaterial>,
    rect_handle: Handle<Mesh>,
}

fn reset_materials(mut q_material: Query<&mut Handle<AABBMaterial>>, store: Res<Store>) {
    for mut mat in q_material.iter_mut() {
        *mat = store.green_material.clone();
    }
}

fn handle_collisions(
    mut ev: EventReader<AABBCollision>,
    store: Res<Store>,
    q: Query<&Children, With<Visualize>>,
    mut q_material: Query<&mut Handle<AABBMaterial>>,
) {
    for AABBCollision { entity1, entity2 } in ev.read() {
        if let Ok(children) = q.get(*entity1) {
            for id in children.iter() {
                if let Ok(mut mat) = q_material.get_mut(*id) {
                    *mat = store.red_material.clone();
                }
            }
        }
        if let Ok(children) = q.get(*entity2) {
            for id in children.iter() {
                if let Ok(mut mat) = q_material.get_mut(*id) {
                    *mat = store.red_material.clone();
                }
            }
        }
    }
}

fn handle_new_viz_requests(
    mut cmd: Commands,
    q: Query<Entity, Added<Visualize>>,
    store: Res<Store>,
) {
    for id in q.iter() {
        cmd.entity(id).with_children(|cmd| {
            cmd.spawn(MaterialMesh2dBundle {
                mesh: store.rect_handle.clone().into(),
                material: store.green_material.clone(),
                ..Default::default()
            });
        });
    }
}

fn update_aabb_transforms(mut q_tr: Query<(&mut Transform, &Parent)>, q: Query<&AABB>) {
    q_tr.par_iter_mut().for_each(|(mut tr, parent)| {
        if let Ok(aabb) = q.get(**parent) {
            tr.scale = (aabb.max - aabb.min).extend(1.0);
        }
    });
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<AABBMaterial>>,
) {
    let mesh = meshes.add(Rectangle::default());
    let green_material = materials.add(AABBMaterial {
        color: LinearRgba::GREEN,
    });
    let red_material = materials.add(AABBMaterial {
        color: LinearRgba::RED,
    });
    cmd.insert_resource(Store {
        green_material,
        red_material,
        rect_handle: mesh,
    });
}

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "aabb.wgsl", Shader::from_wgsl);

        app.add_plugins(Material2dPlugin::<AABBMaterial>::default());

        app.add_systems(Startup, setup)
            .add_systems(
                PostUpdate,
                (handle_new_viz_requests, update_aabb_transforms),
            )
            .add_systems(
                Update,
                (reset_materials, handle_collisions.after(reset_materials)),
            );
    }
}
