// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let offset_strength = settings.intensity;

    // Sample each color channel with an arbitrary shift
    // return vec4<f32>(
    //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
    //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
    //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
    //     1.0
    // );

    // Sample each color channel with a shift
    let red = textureSample(screen_texture, texture_sampler, in.uv).r;
    let green = textureSample(screen_texture, texture_sampler, in.uv).g;
    let blue = textureSample(screen_texture, texture_sampler, in.uv).b;

    // Combine RGB to create grayscale (luminosity method)
    let gray = 0.3 * red + 0.59 * green + 0.11 * blue;

    // Mix the chromatic aberration color with grayscale
    let aberration_color = vec4<f32>(red, green, blue, 1.0);
    let gray_color = vec4<f32>(gray, gray, gray, 1.0);

    // Control the blend with a factor (adjust as desired)
    let blend_factor = 0.5; // 0 for full color, 1 for full grayscale
    return mix(aberration_color, gray_color, blend_factor);
}

