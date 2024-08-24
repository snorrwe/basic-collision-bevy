#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct AABBUniforms {
    color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> aabb: AABBUniforms;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    if mesh.uv.x == 0 || mesh.uv.y == 0 || mesh.uv.x == 1 || mesh.uv.y == 1 {
        return aabb.color;
    }
    return vec4<f32>(0.0);
}
