# ktx

# Useful Commands

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
