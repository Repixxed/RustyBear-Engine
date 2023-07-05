
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};


@vertex
fn vs_main(
    mesh: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = mesh.color;
    out.clip_position = vec4<f32>(mesh.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color.x, in.color.y, in.color.z, 1.0);
}