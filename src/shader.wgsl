struct VertexOutput {
    @builtin(position) my_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vert_main(@builtin(vertex_index) v_index: u32) -> VertexOutput {
    var pos = array(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );

    var colors = array(
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 1.0)
    );

    var output: VertexOutput;

    output.my_pos = vec4<f32>([v_index], 0.0, 1.0);
    output.color = vec4<f32>(colors[v_index], 1.0);

    return output;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
