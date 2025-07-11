# Image Gen

Image processing utilities for texture generation with support for multiple operations:

1. **Depth conversion**: Converts displacement image (white at highest) to a depth map (black at highest)


| Displacement | Depth |
|---|---|
| <img width="256" src="./docs/images/coast_sand_rocks_02_disp_4k.png"> | <img width="256" src="./docs/images/coast_sand_rocks_02_depth_4k.png"> |

## Running

### Depth Conversion

To use from repo:

```bash
cargo run -p image_gen -- depth ./source_disp.png output_depth.png
```

To install so you can use from anywhere:

```bash
cargo install image_gen --path ./tools/image_gen

image_gen depth source_disp.png output_depth.png
```


### Help

```bash
cargo run -p image_gen -- --help
```