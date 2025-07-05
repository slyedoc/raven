// A simple shader that visualizes UV coordinates as colors
// Red channel = U coordinate, Green channel = V coordinate

#import bevy_pbr::{
    pbr_deferred_functions::deferred_output,
    pbr_fragment::pbr_input_from_standard_material,
    prepass_io::{VertexOutput, FragmentOutput},
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    // Create the PBR input
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    
    // Set base color to UV coordinates
    // Red = U coordinate, Green = V coordinate, Blue = 0
    pbr_input.material.base_color = vec4<f32>(in.uv.x, in.uv.y, 0.0, 1.0);
    
    // Send to deferred shader
    return deferred_output(in, pbr_input);
}