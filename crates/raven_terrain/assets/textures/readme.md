# poly haven material setup

Each texture is from polyhaven

## List

For updated list: 
```bash
ls -d ./crates/raven_terrain/assets/textures/*/  
```

- [rock_path](https://polyhaven.com/a/rock_path)
  - <img width="256" src="./rock_path/rock_path_diff_4k.png" />
- [coast_sand_rocks_02](https://polyhaven.com/a/coast_sand_rocks_02)
  - <img width="256" src="./coast_sand_rocks_02/coast_sand_rocks_02_diff_4k.png" />
- [rocky_terrain_03](https://polyhaven.com/a/rocky_terrain_03)
  - <img width="256" src="./rocky_terrain_03/rocky_terrain_03_diff_4k.png" />
- [rock_wall_02](https://polyhaven.com/a/rock_wall_02)
  - <img width="256" src="./rock_wall_02/rock_wall_02_diff_4k.png" />

## Notes

Assumings each will have:

- [name]_ao_[res].png
- [name]_arm_[res].png
- [name]_diff_[res].png
- [name]_disp_[res].png
- [name]_nor_dx_[res].png
- [name]_nor_gl_[res].png
- [name]_rough_[res].png

And a generated depth for displacement in bevy



And convert files with [convert_png_8bit](./tools/convert_png_8bit.sh)

```bash
./tools/convert_png_8bit.sh
```

> Note: For now only 4k res versions


|----|-----|
| ktx name| from image |
|----|-----|
|base_color.ktx| diff_8bit |
|occlusion| ao |
|normal| nor_gl|
|metal_rough| arm|
|rough| rough_4k.png|
|depth_map| depth |

Command to create base_color.ktx


Commands

  ktx create --format R8G8B8A8_UNORM --layers 4 layer0.png layer1.png layer2.png layer3.png output.ktx2