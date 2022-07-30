pub fn blend(current: &[u8], other: &[u8]) -> [u8; 4] {
    // http://stackoverflow.com/questions/7438263/alpha-compositing-algorithm-blend-modes#answer-11163848
    // Taken from the image crate and modified. https://github.com/image-rs/image

    if other[3] == 0 {
        return [current[0], current[1], current[2], current[3]];
    }
    if other[3] == u8::MAX {
        return [other[0], other[1], other[2], other[3]];
    }

    // First, as we don't know what type our pixel is, we have to convert to floats between 0.0 and 1.0
    let max_t: f32 = u8::MAX.into();
    let (bg_r, bg_g, bg_b, bg_a) = (current[0], current[1], current[2], current[3]);
    let (fg_r, fg_g, fg_b, fg_a) = (other[0], other[1], other[2], other[3]);
    let (bg_r, bg_g, bg_b, bg_a): (f32, f32, f32, f32) = (
        bg_r as f32 / max_t,
        bg_g as f32 / max_t,
        bg_b as f32 / max_t,
        bg_a as f32 / max_t,
    );
    let (fg_r, fg_g, fg_b, fg_a): (f32, f32, f32, f32) = (
        fg_r as f32 / max_t,
        fg_g as f32 / max_t,
        fg_b as f32 / max_t,
        fg_a as f32 / max_t,
    );

    // Work out what the final alpha level will be
    let alpha_final = bg_a + fg_a - bg_a * fg_a;
    if alpha_final == 0.0 {
        return [current[0], current[1], current[2], current[3]];
    };

    // We premultiply our channels by their alpha, as this makes it easier to calculate
    let (bg_r_a, bg_g_a, bg_b_a) = (bg_r * bg_a, bg_g * bg_a, bg_b * bg_a);
    let (fg_r_a, fg_g_a, fg_b_a) = (fg_r * fg_a, fg_g * fg_a, fg_b * fg_a);

    // Standard formula for src-over alpha compositing
    let (out_r_a, out_g_a, out_b_a) = (
        fg_r_a + bg_r_a * (1.0 - fg_a),
        fg_g_a + bg_g_a * (1.0 - fg_a),
        fg_b_a + bg_b_a * (1.0 - fg_a),
    );

    // Unmultiply the channels by our resultant alpha channel
    let (out_r, out_g, out_b) = (
        out_r_a / alpha_final,
        out_g_a / alpha_final,
        out_b_a / alpha_final,
    );

    // Cast back to our initial type on return
    [
        (max_t * out_r).round() as u8,
        (max_t * out_g).round() as u8,
        (max_t * out_b).round() as u8,
        (max_t * alpha_final) as u8,
    ]
}
