#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

#ifdef MULTISAMPLED
@group(0) @binding(0) var in_texture: texture_multisampled_2d<f32>;
#else
@group(0) @binding(0) var in_texture: texture_2d<f32>;
@group(0) @binding(1) var in_sampler: sampler;
#endif

@fragment
fn fs_main(
#ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
#endif
    in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
#ifdef MULTISAMPLED
    return textureLoad(in_texture, vec2<i32>(in.position.xy), sample_index);
#else
    return textureSample(in_texture, in_sampler, in.uv);
#endif
}
