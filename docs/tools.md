# Tools



## Images

Lastest bevy related info: 

[bevy issue 14671](https://github.com/bevyengine/bevy/issues/14671) 


I have used basicu and little [nvtt](https://developer.nvidia.com/gpu-accelerated-texture-compression), but had most luck with ktx


### KTX-Software

For image processing - [KTX Release](https://github.com/KhronosGroup/KTX-Software/releases)

```bash
sudo apt install ktx-software
```

Creating skybox textures:

```bash
ktx create --format R8G8B8A8_UNORM --cubemap right.png left.png top.png bot.png front.png back.png skybox.ktx2
```

Create mipmaps

```bash
ktx create test.png test.ktx2 --format R8G8B8A8_SRGB --generate-mipmap
```

convert "$file" -depth 8 "${file%.png}_8bit.png"

Create texture array

R8G8B8A8_SRGB

ktx create --format R8G8B8A8_SRGB --layers 4 $(find ./crates/raven_terrain/assets/textures -name "*_diff_4k.png" | sort) base_color.ktx2