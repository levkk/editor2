
// Vertices with position and color.
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.color = vertex.color;
    out.position = vec4<f32>(vertex.position, 1.0);

    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(
    in: VertexOutput
) -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4<f32>(in.color, 1.0);

    return out;
}