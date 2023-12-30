use xxhash_rust::xxh64::xxh64;

const SIZE: u32 = 1000;

fn value_noise(x: u32, y: u32) -> f64 {
    let hash = xxh64(&[x.to_le_bytes(), y.to_le_bytes()].concat(), 0);
    let zero_to_one = (hash as f64) / (u64::MAX as f64);
    // let neg_one_to_one = zero_to_one * 2.0 - 1.0;
    zero_to_one
    // neg_one_to_one
}

fn noise_at_grid_nodes(x0: u32, y0: u32, x1: u32, y1: u32) -> (f64, f64, f64, f64) {
    let noise_00 = value_noise(x0, y0);
    let noise_01 = value_noise(x0, y1);
    let noise_10 = value_noise(x1, y0);
    let noise_11 = value_noise(x1, y1);
    (noise_00, noise_01, noise_10, noise_11)
}

fn grid_origin(frequency: u32, x: u32, y: u32) -> (u32, u32) {
    let low_x = x / frequency * frequency;
    let low_y = y / frequency * frequency;
    (low_x, low_y)
}

fn linear_interpolate(x: u32, x0: u32, x1: u32, noise_0: f64, noise_1: f64) -> f64 {
    let ratio = (x - x0) as f64 / (x1 - x0) as f64;
    noise_0 * (1.0 - ratio) + noise_1 * ratio
}

fn smooth_interpolate(x: u32, x0: u32, x1: u32, noise_0: f64, noise_1: f64) -> f64 {
    let ratio = (x - x0) as f64 / (x1 - x0) as f64;
    (noise_1 - noise_0) * (3.0 - 2.0 * ratio) * ratio * ratio + noise_0
}

fn smoother_interpolate(x: u32, x0: u32, x1: u32, noise_0: f64, noise_1: f64) -> f64 {
    let ratio = (x - x0) as f64 / (x1 - x0) as f64;
    (noise_1 - noise_0) * (6.0 * ratio * ratio * ratio * ratio * ratio - 15.0 * ratio * ratio * ratio * ratio + 10.0 * ratio * ratio * ratio) + noise_0
}

fn interpolate(x: u32, y: u32, x0: u32, y0: u32, frequency: u32, noise_00: f64, noise_01: f64, noise_10: f64, noise_11: f64) -> f64 {
    // let noise_0 = linear_interpolate(x, x0, x0 + frequency, noise_00, noise_10);
    // let noise_1 = linear_interpolate(x, x0, x0 + frequency, noise_01, noise_11);
    // linear_interpolate(y, y0, y0 + frequency, noise_0, noise_1)

    let noise_0 = smooth_interpolate(x, x0, x0 + frequency, noise_00, noise_10);
    let noise_1 = smooth_interpolate(x, x0, x0 + frequency, noise_01, noise_11);
    smooth_interpolate(y, y0, y0 + frequency, noise_0, noise_1)

    // let noise_0 = smoother_interpolate(x, x0, x0 + frequency, noise_00, noise_10);
    // let noise_1 = smoother_interpolate(x, x0, x0 + frequency, noise_01, noise_11);
    // smoother_interpolate(y, y0, y0 + frequency, noise_0, noise_1)
}

fn perlin(frequency: u32, amplitude: f64, x: u32, y: u32) -> f64 {
    let (x0, y0) = grid_origin(frequency, x, y);
    let (x1, y1) = (x0 + frequency, y0 + frequency);

    let (noise_00,
        noise_01,
        noise_10,
        noise_11) = noise_at_grid_nodes(x0, y0, x1, y1);

    interpolate(x, y, x0, y0, frequency, noise_00, noise_01, noise_10, noise_11) * amplitude
}

fn multi_perlin(octaves: u32, x: u32, y: u32) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1000;
    let mut amplitude = 0.1;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += perlin(frequency, amplitude, x, y);
        max_value += amplitude;
        amplitude /= 1.1;
        frequency = (frequency as f64 / 1.8).max(1.0f64) as u32;
    }

    let turbulence = total / max_value;
    // let turbulence = f64::sin(hehe * std::f64::consts::PI);
    f64::abs(f64::sin((((x as f64) * 0.015 + (y as f64) * 0.01) + turbulence * 10.0)))
}

fn main() {
    let mut imgbuf = image::ImageBuffer::new(SIZE, SIZE);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let c = (multi_perlin(9, x, y) * 255.0) as u8;
        *pixel = image::Rgb([c, c, c]);
    }

    imgbuf.save("perlin.png").unwrap();
}
