struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix0: vec4<f32>,
    @location(6) model_matrix1: vec4<f32>,
    @location(7) model_matrix2: vec4<f32>,
    @location(8) model_matrix3: vec4<f32>,
};

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vert_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix0,
        instance.model_matrix1,
        instance.model_matrix2,
        instance.model_matrix3
    );

    var out: VertexOutput;

    out.clip_pos = camera.view_proj * model_matrix * vec4<f32>(model.pos, 1);
    out.tex_coords = vec2<f32>(model.tex_coords.x, 1.0 - model.tex_coords.y);

    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
