# PNG Interlace

Converts interlaced PNG files to non-interlaced format using the Rust image crate.

## Features

- Convert interlaced PNG to non-interlaced format
- Preserves image quality and metadata
- Simple command-line interface
- No external dependencies required

## Running

To use from repo:

```bash
cargo run -p png_interlace -- input.png output.png
```

To install globally:

```bash
cargo install --path ./tools/png_interlace

png_interlace input.png output.png
```

## Examples

Convert interlaced PNG to non-interlaced:
```bash
png_interlace cube_normal.png cube_normal_fixed.png
```