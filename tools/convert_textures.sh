  # Convert all to 8-bit RGB
res="4k"



for file in $(find ./crates/raven_terrain/assets/textures -name "*_disp_${res}.png"); do
  new_file="${file%_disp_${res}.png}_depth_${res}.png"
  echo "Converted ${file} to ${new_file}"
  cargo run -p depth_gen "${file}" "${new_file}"  
done

  
# Convert all images to sRGB colorspace with consistent primaries
for file in $(find ./crates/raven_terrain/assets/textures -name "*_diff_4k.png"); do
    convert "$file" -colorspace sRGB -depth 8 "${file%.png}_f.png"
done
  
# Create a texture array for base color

ktx create \
  --format R8G8B8A8_SRGB \
  --layers 4 \
  --assign-primaries bt709 \
  --assign-tf srgb \
  $(find ./crates/raven_terrain/assets/textures -name "*_diff_4k_f.png" | sort) \
  ./crates/raven_terrain/assets/textures/base_color.ktx2