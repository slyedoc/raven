# Depth Gen

Converts displacment image (white at highest) to a depth map (black at heighest)

| Displacement | Depth |
|---|---|
| <img width="256" src="./docs/images/coast_sand_rocks_02_disp_4k.png"> | <img width="256" src="./docs/images/coast_sand_rocks_02_depth_4k.png"> |

## Running

To use from repo:

```
cargo run -p depth_gen -- ./source.png output.png
```

To install so you can use form anywhere:

```bash
cargo install depth_gen --path ./tools/depth_gen

depth_gen source.png output.png
```