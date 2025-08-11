
# rough_piet

[![Crates.io](https://img.shields.io/crates/v/rough_piet.svg)](https://crates.io/crates/rough_piet)
[![Documentation](https://docs.rs/rough_piet/badge.svg)](https://docs.rs/rough_piet)
[![License](https://img.shields.io/github/license/orhanbalci/rough-rs.svg)](https://github.com/orhanbalci/rough-rs/LICENSE)

<!-- cargo-sync-readme start -->


This crate is an adapter crate between [roughr](https://github.com/orhanbalci/rough-rs/main/roughr) and
[vello](https://github.com/linebender/vello) crates. Converts from roughr drawing
primitives to vello's Scene types. Also has convenience traits for drawing onto vello scenes. For more detailed
information you can check roughr crate.

Below examples are output of [rough_vello](https://github.com/orhanbalci/rough-rs/tree/main/rough_vello) adapter.

## 📦 Cargo.toml

```toml
[dependencies]
rough_vello = "0.1"
```

## 🔧 Example

### Rust Logo

```rust
use rough_vello::VelloGenerator;
use vello::Scene;
use palette::Srgba;
use roughr::core::{FillStyle, OptionsBuilder};

let options = OptionsBuilder::default()
    .stroke(Srgba::from_components((114u8, 87u8, 82u8, 255u8)).into_format())
    .fill(Srgba::from_components((254u8, 246u8, 201u8, 255)).into_format())
    .fill_style(FillStyle::Hachure)
    .fill_weight(1.0)
    .bowing(0.8)
    .build()
    .unwrap();

let generator = VelloGenerator::new(options);
let rust_logo_svg_path = "..."; // SVG path data for the Rust logo
let rust_logo_drawing = generator.path::<f32>(rust_logo_svg_path);

let mut scene = Scene::new();
rust_logo_drawing.draw(&mut scene);
```

### 🖨️ Output Rust Logo
![rust_logo](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/rough_vello/assets/rust_logo.png)

## Filler Implementation Status
- [x] Hachure
- [x] Zigzag
- [x] Cross-Hatch
- [x] Dots
- [x] Dashed
- [x] Zigzag-Line

## 🔭 Examples

For more examples have a look at the
[examples](https://github.com/orhanbalci/rough-rs/tree/main/rough_vello/examples) folder.

<!-- cargo-sync-readme end -->

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
