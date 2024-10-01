use crate::{demo::DemoApp, init::State};

pub fn draw_circle(state: &mut State<DemoApp>) {
    let color = &state.game_context.color;
    let width = state.texture_extent.width as f32;
    let height = state.texture_extent.height as f32;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = width.min(height) / 4.0;

    for y in 0..state.texture_extent.height as usize {
        for x in 0..state.texture_extent.width as usize {
            let index = (y * state.texture_extent.width as usize + x) * 4;

            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();

            let (r, g, b) = color.get_components();
            if distance <= radius {
                // Inside the circle: red
                state.pixels[index] = r; // Red channel
                state.pixels[index + 1] = g; // Green channel
                state.pixels[index + 2] = b; // Blue channel
                state.pixels[index + 3] = 255; // Alpha channel
            } else {
                // Outside the circle: black
                state.pixels[index] = 0; // Red channel
                state.pixels[index + 1] = 0; // Green channel
                state.pixels[index + 2] = 0; // Blue channel
                state.pixels[index + 3] = 255; // Alpha channel
            }
        }
    }
}

pub fn draw_square(state: &mut State<DemoApp>) {
    let color = &state.game_context.color;
    let width = state.texture_extent.width as f32;
    let height = state.texture_extent.height as f32;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let half_side = width.min(height) / 4.0;

    for y in 0..state.texture_extent.height as usize {
        for x in 0..state.texture_extent.width as usize {
            let index = (y * state.texture_extent.width as usize + x) * 4;

            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;

            let (r, g, b) = color.get_components();
            if dx.abs() <= half_side && dy.abs() <= half_side {
                // Inside the square: red
                state.pixels[index] = r; // Red channel
                state.pixels[index + 1] = g; // Green channel
                state.pixels[index + 2] = b; // Blue channel
                state.pixels[index + 3] = 255; // Alpha channel
            } else {
                // Outside the square: black
                state.pixels[index] = 0; // Red channel
                state.pixels[index + 1] = 0; // Green channel
                state.pixels[index + 2] = 0; // Blue channel
                state.pixels[index + 3] = 255; // Alpha channel
            }
        }
    }
}

pub fn draw_triangle(state: &mut State<DemoApp>) {
    let color = &state.game_context.color;
    let width = state.texture_extent.width as f32;
    let height = state.texture_extent.height as f32;

    // verts
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let half_base = width.min(height) / 4.0;
    let height_triangle = half_base * (3.0f32).sqrt();

    let v0 = (center_x - half_base, center_y + height_triangle / 2.0); // bl 
    let v1 = (center_x + half_base, center_y + height_triangle / 2.0); // br
    let v2 = (center_x, center_y - height_triangle / 2.0); // t

    // Function to calculate barycentric coordinates
    // helper
    fn barycentric_coords(
        x: f32,
        y: f32,
        v0: (f32, f32),
        v1: (f32, f32),
        v2: (f32, f32),
    ) -> (f32, f32, f32) {
        let denom = (v1.1 - v2.1) * (v0.0 - v2.0) + (v2.0 - v1.0) * (v0.1 - v2.1);
        let lambda1 = ((v1.1 - v2.1) * (x - v2.0) + (v2.0 - v1.0) * (y - v2.1)) / denom;
        let lambda2 = ((v2.1 - v0.1) * (x - v2.0) + (v0.0 - v2.0) * (y - v2.1)) / denom;
        let lambda3 = 1.0 - lambda1 - lambda2;
        (lambda1, lambda2, lambda3)
    }

    for y in 0..state.texture_extent.height as usize {
        for x in 0..state.texture_extent.width as usize {
            let index = (y * state.texture_extent.width as usize + x) * 4;
            let xf = x as f32;
            let yf = y as f32;

            let (r, g, b) = color.get_components();
            let (lambda1, lambda2, lambda3) = barycentric_coords(xf, yf, v0, v1, v2);

            // If all barycentric coordinates are between 0 and 1, the point is inside the triangle
            if lambda1 >= 0.0
                && lambda1 <= 1.0
                && lambda2 >= 0.0
                && lambda2 <= 1.0
                && lambda3 >= 0.0
                && lambda3 <= 1.0
            {
                // Inside the triangle: fill with color
                state.pixels[index] = r; // Red channel
                state.pixels[index + 1] = g; // Green channel
                state.pixels[index + 2] = b; // Blue channel
                state.pixels[index + 3] = 255; // Alpha channel
            } else {
                // Outside the triangle: black
                state.pixels[index] = 0; // Red channel
                state.pixels[index + 1] = 0; // Green channel
                state.pixels[index + 2] = 0; // Blue channel
                state.pixels[index + 3] = 255; // Alpha channel
            }
        }
    }
}
