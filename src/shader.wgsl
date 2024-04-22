struct VertexOutput {
    @builtin(position) my_pos: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vert_main(@builtin(vertex_index) v_index: u32) -> VertexOutput {
    var pos = array(
        vec2f(0.0, 0.5),
        vec2f(-0.5, -0.5),
        vec2f(0.5, -0.5)
    );

    var colors = array(
        vec3f(1.0, 0.0, 0.0),
        vec3f(0.0, 1.0, 0.0),
        vec3f(0.0, 0.0, 1.0)
    );

    var output: VertexOutput;

    output.my_pos = vec4f(pos[v_index], 0.0, 1.0);
    output.color = vec4f(colors[v_index], 1.0);

    return output;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
