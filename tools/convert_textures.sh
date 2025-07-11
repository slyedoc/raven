#!/bin/bash

# Yes! this script redoes alot of work, but want adding new textures to be easy

res="4k"
texture_dir="./crates/raven_terrain/assets/textures"

# reverses displacement maps to depth maps and converts base_color images to sRGB colorspace and same bit depth
fix_images_from_polyhaven() {
  # Convert displacement maps to depth maps
  for file in $(find ${texture_dir} -name "*_disp_${res}.png"); do
    new_file="${file%_disp_${res}.png}_depth_${res}.png"
    echo "Converting $(basename "${file}") to $(basename "${new_file}")"
    cargo run -p image_gen -- depth "${file}" "${new_file}"  
  done

  # ARM files are already in the correct format for Bevy (metallic in blue, roughness in green)
  # No conversion needed
    
  # Convert all base_color images to sRGB colorspace and 8bit
  for file in $(find ${texture_dir} -name "*_diff_4k.png"); do
    echo "Converting $(basename "${file}") to sRGB colorspace"
      convert "$file" -colorspace sRGB -depth 8 "${file%_diff_4k.png}_base_4k.png"
  done
}

# Create texture arrays for all texture types
create_texture_array() {
  local format=$1
  local ktx_name=$2
  local pattern=$3
  
  local files=$(find ${texture_dir} -name "*_${pattern}.png" | sort)
  
  if [ -n "$files" ]; then
    echo "Creating texture array for ${ktx_name} textures"
    ktx create \
      --format ${format} \
      --layers 4 \
      --assign-primaries bt709 \
      --assign-tf srgb \
      --generate-mipmap \
      $files \
      ${texture_dir}/${ktx_name}.ktx2
  else
    echo "Warning: No files found for ${ktx_name} (pattern: *_${pattern}.png)"
  fi
}


# fix_images_from_polyhaven

create_texture_array R8G8B8A8_SRGB "base_color" "base_4k"
create_texture_array R8G8B8A8_SRGB "occlusion" "ao_4k"
create_texture_array R8G8B8A8_SRGB "normal" "nor_gl_4k"
create_texture_array R8G8B8A8_SRGB "metal_rough" "arm_4k"
create_texture_array R8G8B8A8_SRGB "depth_map" "depth_4k"