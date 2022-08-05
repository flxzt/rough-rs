//! This example shows painting a rough rectangle using common-piet crate and
//! kurbo rough shape generator

use piet::{Color, RenderContext};
use piet_common::kurbo::Rect;
use piet_common::Device;
use rough_piet::KurboGenerator;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// For now, assume pixel density (dots per inch)
const DPI: f64 = 96.;

/// Feature "png" needed for save_to_file() and it's disabled by default for optional dependencies
/// cargo run --example mondrian --features png
fn main() {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut rc = bitmap.render_context();
    let generator = KurboGenerator::default();
    let rect_width = 1000.0;
    let rect_height = 500.0;
    let rect = generator.rectangle::<f32>(
        (WIDTH as f32 - rect_width) / 2.0,
        (HEIGHT as f32 - rect_height) / 2.0,
        rect_width,
        rect_height,
    );
    let background_color = Color::from_hex_str("96C0B7").unwrap();
    let stroke_color = Color::from_hex_str("725752").unwrap();
    let sketch_color = Color::from_hex_str("FEF6C9").unwrap();

    rc.fill(
        Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
        &background_color,
    );

    for path in rect.iter() {
        rc.stroke(path, &stroke_color, 0.01 * DPI);
    }

    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("rectangle.png")
        .expect("file save error");
}
