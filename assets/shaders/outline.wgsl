@group(1) @binding(0)
var render_texture: texture_2d<f32>;
@group(1) @binding(1)
var render_texture_sampler: sampler;

fn back_to_uv(pixel_pos: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(pixel_pos.x / 320.0, pixel_pos.y / 180.0);
}

@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(1.0 - uv.x, uv.y); // DIRTY DIRTY UV UNFUCK

    let pixel_pos = uv * vec2<f32>(320.0, 180.0);
    let base_color = textureSample(render_texture, render_texture_sampler, uv);

    if (base_color.a > 0.0) {
        return base_color;
    }

    for (var y: i32 = -1; y < 2; y++) {
        for (var x: i32 = -1; x < 2; x++) {
            if (x == y) { continue; }

            let offset_pos = back_to_uv(pixel_pos + vec2<f32>(f32(x), f32(y)));

            let color = textureSampleLevel(render_texture, render_texture_sampler, offset_pos, 0.0);

            if (color.a > 0.0) {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
        }
    }

    return base_color;
}
