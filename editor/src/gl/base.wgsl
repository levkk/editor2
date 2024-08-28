
// Vertices with position and color.
struct VertexInput {
    @location(0) position: vec3<f32>,
    //@location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    //@location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}