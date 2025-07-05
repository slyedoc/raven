# Raven Terrain

This is a fresh start for terrain generation in bevy

## Notes

Ideas from my previous projects

- [Simple Compute Shader](https://github.com/slyedoc/bevy_sly_compute/blob/2d7f067ba47c29dd068527398b2e7f14e16ef799/examples/terrain.rs#L213)
- [Full Blown Compute Workflow](https://github.com/slyedoc/orbit/blob/main/crates/bevy_terrain/src/lib.rs)

While ideal to use the gpu, building workflow like this was pain, and the fork of bevy for macro mod was a pain, shouldn't need that now days, see [gpu_readback](https://github.com/bevyengine/bevy/blob/e6ba9a6d181ad20320e4dadc26575b7f5a6c6f94/examples/shader/gpu_readback.rs#L22)


Other resources
- [SimonDev](https://www.youtube.com/playlist?list=PLRL3Z3lpLmH3PNGZuDNf2WXnLTHpN9hXy) and [repos](https://github.com/simondevyoutube?tab=repositories&q=terrain&type=&language=&sort=) 
- [bevy_triplanar_splatting](https://github.com/bonsairobo/bevy_triplanar_splatting)
- [Yu266426/Bevy-Erosion](https://github.com/Yu266426/Bevy-Erosion/) 