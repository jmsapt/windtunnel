#import bevy_pbr::forward_io::VertexOutput

struct FragmentInput {
    @builtin(position) frag_coord: vec4<f32>,
}

// struct GridUniforms {
//     x_bounds: vec2<f32>,
//     y_bounds: vec2<f32>,
//     z_bounds: vec2<f32>,
//     x_divisions: u32,
//     y_divisions: u32,
//     z_divisions: u32,
// };


// @group(2) @binding(0) var<uniform> gridUniforms: GridUniforms;
// @group(2) @binding(1) var<storage> velocity: vec3<f32>;
@group(1) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(1) var<storage> pressure: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
    // color: vec4<f32>,
    // pressure: f32,
) -> @location(0) vec4<f32> {
    return color;
}
