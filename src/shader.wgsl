@vertex
fn vert_main(@builtin(vertex_index) vertex_index: u32) -> @location(0) vec4<f32> {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );

    let colors = array<vec4<f32>, 3>(
        vec4<f32>(1.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0)
    );

    return vec4<f32>(colors[vertex_index], 1.0);
}

@fragment
fn frag_main(@location(0) color: vec4<f32>) -> @location(1) vec4<f32> {
    return vec4<f32>(color, 1.0);
}