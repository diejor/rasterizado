struct VOutput {
    @builtin(position) my_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vert_main(@builtin(vertex_index) v_index: u32) -> VOutput {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );

    let colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0)
    );

    var output: VOutput;

    output.my_pos = vec4<f32>(positions[v_index], 0.0, 1.0);
    output.color = vec4<f32>(colors[v_index], 1.0);

    return output;
}

@fragment
fn frag_main(in: VOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
