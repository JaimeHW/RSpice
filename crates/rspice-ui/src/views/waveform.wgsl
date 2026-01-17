// Waveform rendering shader
// Uses line strip topology with color interpolation

struct ViewUniforms {
    x_scale: f32,
    x_offset: f32,
    y_scale: f32,
    y_offset: f32,
};

@group(0) @binding(0)
var<uniform> view: ViewUniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply view transform (pan/zoom)
    let x = in.position.x * view.x_scale + view.x_offset;
    let y = in.position.y * view.y_scale + view.y_offset;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = in.color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
