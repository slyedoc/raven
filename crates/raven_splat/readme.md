# Raven Splat


# Credit

Started from [bevy_triplanar_splatting](https://github.com/bonsairobo/bevy_triplanar_splatting)

It was based on bevy 0.14, its assets used odd resolution for array_textures and I couldnt load them on linux, think author was on a mac.

When it was written bevy didnt have `ExtendedMaterial` yet so alot of `StandardMaterial` is was duplicated.  I also wanted to support GPU-Driven Rendering.

> - Ben Golus, ["Normal Mapping for a Triplanar
>   Shader"](https://bgolus.medium.com/normal-mapping-for-a-triplanar-shader-10bf39dca05a)
> - Inigo Quilez, ["Biplanar
>   Mapping"](https://iquilezles.org/articles/biplanar/)
> - Colin Barré-Brisebois and Stephen Hill, ["Blending in
>   Detail"](https://blog.selfshadow.com/publications/blending-in-detail/)
>
> # Road Map
>
> - [ ] per-layer uniform constants (e.g. "emissive", "metallic", etc.)
> - [ ] support different texture per plane, using more layers
> - [ ] blend materials using depth/height map
>   - see ["Advanced Terrain Texture
>     Splatting"](https://www.gamedeveloper.com/programming/advanced-terrain-texture-splatting)


> TODO: Make this a PR