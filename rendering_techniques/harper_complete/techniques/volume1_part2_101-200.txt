Roughness-dependent temporal filtering for specular reflections.

**Filter Kernel:**
```
history_weight = pow(roughness, 2) * base_weight  // Rougher = more history
spatial_sigma = roughness * max_sigma
temporal_filter(history, current, history_weight, spatial_sigma)
```

---

## 080. Stochastic Frame Scheduling (SFS)
**Novelty Class:** Novel Combination

Probabilistic selection of which effects to update each frame.

**Scheduling:**
```
update_probability[effect] = importance[effect] * time_since_update[effect]
if random() < update_probability[effect]:
    update(effect)
    time_since_update[effect] = 0
```

---

## 081. Depth-Peeled Temporal Layers (DPTL)
**Novelty Class:** Patent-Worthy Invention

Maintains separate history for each depth layer.

**Layers:**
```
for each pixel:
    layer = classify_depth_layer(depth)
    history[layer] = reproject(history[layer], motion_vector[layer])
    result = blend_layers(history, current, layer_weights)
```

---

## 082. Temporally Stable Derivatives (TSD)
**Novelty Class:** Significant Improvement

Filters screen-space derivatives for stable normal mapping.

**Filtering:**
```
dFdx_filtered = lerp(dFdx_current, dFdx_history, stability_weight)
dFdy_filtered = lerp(dFdy_current, dFdy_history, stability_weight)
stability_weight = compute_motion_stability(motion_vectors)
```

---

## 083. Async History Updates (AHU)
**Novelty Class:** Novel Combination

Background thread updates history buffers between frames.

**Pipeline:**
```
Frame N: Render with history from N-2
         Async: Process N-1 history update
Frame N+1: Use freshly updated history
```

---

## 084. Motion-Compensated LOD (MCLOD)
**Novelty Class:** Significant Improvement

LOD selection accounts for motion blur hiding detail loss.

**LOD Adjustment:**
```
motion_blur_factor = motion_magnitude / shutter_speed
effective_resolution = base_resolution * (1 - motion_blur_factor)
lod_bias = log2(effective_resolution / base_resolution)
```

---

## 085. Temporal Coherence Metrics (TCM)
**Novelty Class:** Significant Improvement

Real-time measurement of temporal stability for quality control.

**Metrics:**
```
flicker_score = variance(luminance_history) / mean(luminance_history)
ghosting_score = detect_trailing_artifacts(motion_compensated_diff)
quality_score = 1 - (flicker_score + ghosting_score) / 2
```

---

## 086. Progressive Temporal Refinement (PTR)
**Novelty Class:** Patent-Worthy Invention

Each frame adds refinement samples that persist across time.

**Accumulation:**
```
sample_pattern[frame % pattern_length] = current_samples
accumulated = weighted_sum(sample_pattern)
weights = distance_based_falloff(frame_age)
```

---

## 087. History Buffer Compression (HBC)
**Novelty Class:** Significant Improvement

Lossy compression of history buffers to reduce bandwidth.

**Compression:**
```
compressed_history = encode_YCoCg(history)
subsample_chroma(compressed_history, 4:2:0)
quantize(compressed_history, bits=10)
// 50% bandwidth reduction
```

---

## 088. Velocity-Aware Mipmapping (VAM)
**Novelty Class:** Novel Combination

Mip level selection based on both distance and motion.

**Mip Bias:**
```
motion_mip_bias = log2(velocity_screen_space / texel_density)
final_mip = distance_mip + max(0, motion_mip_bias)
```

---

## 089. Temporal Noise Recycling (TNR)
**Novelty Class:** Significant Improvement

Reuses noise patterns with temporal offset for stable stochastic effects.

**Recycling:**
```
noise_offset = hash(frame_index) * golden_ratio
recycled_noise = blue_noise[(uv + noise_offset) % texture_size]
```

---

## 090. Frame Rate Independent Physics (FRIP)
**Novelty Class:** Significant Improvement

Decouples physics simulation from render framerate with interpolation.

**Interpolation:**
```
physics_update(fixed_dt)  // 120Hz
render_alpha = (render_time - physics_time) / fixed_dt
interpolated_transform = lerp(previous_transform, current_transform, render_alpha)
```

---

## 091. Temporal Anti-Aliasing for Particles (TAAP)
**Novelty Class:** Novel Combination

Modified TAA for semi-transparent particle systems.

**Modification:**
```
particle_velocity = derive_from_simulation()
separate_history_blend_for_particles(lower_history_weight)
depth_agnostic_reprojection(particle_depths)
```

---

## 092. Predictive Geometry Streaming (PGS)
**Novelty Class:** Significant Improvement

Motion prediction for geometry LOD prefetching.

**Prediction:**
```
predicted_position = current_position + velocity * lookahead_time
predicted_view_frustum = compute_frustum(predicted_position)
prefetch_lods_for_frustum(predicted_view_frustum)
```

---

## 093. Temporal Supersampling Grid (TSG)
**Novelty Class:** Patent-Worthy Invention

Reconstructs 4K from 1080p using optimized jitter pattern.

**Pattern:**
```
16-frame Halton sequence covering 2×2 supersampling
Reconstruction kernel: Lanczos with temporal weights
Result: 4K quality from 1080p rendering cost
```

---

## 094. History Invalidation Masks (HIM)
**Novelty Class:** Significant Improvement

Per-pixel masks indicating valid history regions.

**Mask Generation:**
```
mask = valid_reproject && !disoccluded && color_coherent && !object_id_changed
use_history_where(mask == 1)
use_fallback_where(mask == 0)
```

---

## 095. Temporal Ray Budget Allocation (TRBA)
**Novelty Class:** Patent-Worthy Invention

Amortizes ray tracing across frames with importance-based scheduling.

**Allocation:**
```
pixel_priority = variance * time_since_update * screen_importance
ray_budget_per_pixel = total_rays * (pixel_priority / sum_priorities)
distribute_rays_across_frames(ray_budget_per_pixel)
```

---

# CATEGORY IV: POST-PROCESSING EFFECTS (Techniques 096-130)

## 096. Spectral Bloom Decomposition (SBD)
**Novelty Class:** Patent-Worthy Invention

Wavelength-dependent bloom with chromatic dispersion.

**Decomposition:**
```
for wavelength in [400nm, 500nm, 600nm, 700nm]:
    bloom[wavelength] = gaussian_blur(extract_wavelength(hdr, wavelength), σ(wavelength))
    σ(wavelength) = base_σ * (700 / wavelength)  // Red spreads more
result = integrate_spectrum(bloom)
```

---

## 097. Procedural Lens Flare System (PLFS)
**Novelty Class:** Significant Improvement

Real-time flare generation from light source analysis.

**Generation:**
```
for each bright_source:
    flare_elements = generate_from_lens_model(source_position, brightness)
    elements: [ghosts, halo, starburst, anamorphic_streak]
    each element: procedural_texture * color * falloff
```

---

## 098. Physical Chromatic Aberration (PCA)
**Novelty Class:** Significant Improvement

Dispersion model based on actual lens materials.

**Dispersion:**
```
for wavelength in visible_spectrum:
    refractive_index = sellmeier_equation(wavelength, lens_material)
    uv_offset = compute_refraction_offset(uv, refractive_index)
    result += sample_at_wavelength(uv_offset, wavelength)
```

---

## 099. Depth-Aware Sharpening (DAS)
**Novelty Class:** Significant Improvement

Sharpening strength varies with depth for focus simulation.

**Sharpening:**
```
depth_factor = gaussian(depth - focus_distance, focus_sigma)
sharpening_strength = base_strength * depth_factor
result = sharpen(input, sharpening_strength)
```

---

## 100. Cinematic Depth of Field (CDOF)
**Novelty Class:** Patent-Worthy Invention

Scattering-based DoF with physically accurate bokeh.

**Algorithm:**
```
1. Compute CoC from depth and aperture
2. Classify pixels: foreground, focal, background
3. Scatter foreground (near) over focal/background
4. Gather background (far) blur
5. Composite with proper alpha
```

**Bokeh Shape:**
```
shape = aperture_blade_count, curvature, rotation
optical_vignette = radial_falloff + cat_eye_factor
```

---

## 101. Perceptual Color Grading (PCG)
**Novelty Class:** Significant Improvement

Color manipulation in perceptual color spaces.

**Pipeline:**
```
1. Convert to Oklab
2. Apply luminance curve
3. Apply chroma adjustments
4. Apply hue shifts
5. Convert back to RGB
```

**Advantage:** Linear perceptual changes, no hue shifts during saturation adjustments.

---

## 102. HDR Histogram Tonemapping (HHT)
**Novelty Class:** Novel Combination

Tonemapping curve derived from scene histogram.

**Curve Generation:**
```
histogram = compute_log_luminance_histogram(hdr_input)
cdf = cumulative_distribution(histogram)
tonemap_curve = smooth(cdf)  // S-curve smoothing
apply_curve(hdr_input, tonemap_curve)
```

---

## 103. Anamorphic Lens Simulation (ALS)
**Novelty Class:** Significant Improvement

Complete anamorphic look including squeeze, flares, and bokeh.

**Components:**
```
1. Horizontal squeeze (1.33x or 2x)
2. Horizontal streak flares on bright sources
3. Oval bokeh shapes
4. Enhanced lens breathing
5. Characteristic barrel distortion
```

---

## 104. Film Grain Synthesis (FGS)
**Novelty Class:** Significant Improvement

Physically-based film grain matching real stock characteristics.

**Synthesis:**
```
grain_density = film_stock_iso / current_exposure
grain_size = base_size * pow(iso / 100, 0.3)
grain = generate_coherent_noise(uv, grain_size, density)
grain_color = [1.0, 0.95, 0.9]  // Slight yellow tint for accuracy
```

---

## 105. Atmospheric Scattering Post-Process (ASPP)
**Novelty Class:** Novel Combination

Screen-space atmospheric effects without volumetric rendering.

**Algorithm:**
```
1. Reconstruct world position from depth
2. Ray march to camera in screen space
3. Accumulate scattering using analytic formula
4. Apply height fog falloff
```

**Performance:** 0.4ms vs 2ms for volumetric approach.

---

## 106. Lens Distortion Correction (LDC)
**Novelty Class:** Significant Improvement

GPU-efficient Brown-Conrady distortion model.

**Distortion:**
```
r = distance_from_center(uv)
distorted_r = r * (1 + k1*r² + k2*r⁴ + k3*r⁶)
tangential = [p1*(r² + 2*x²) + 2*p2*x*y,
              p2*(r² + 2*y²) + 2*p1*x*y]
distorted_uv = apply_distortion(uv, distorted_r, tangential)
```

---

## 107. Halation Effect (HE)
**Novelty Class:** Novel Combination

Film halation simulation—red glow around bright objects.

**Simulation:**
```
bright_mask = threshold(luminance, halation_threshold)
halation = gaussian_blur(bright_mask, large_σ)
halation_color = [1.0, 0.3, 0.1]  // Red-orange
result += halation * halation_color * intensity
```

---

## 108. Vignette with Optical Accuracy (VOA)
**Novelty Class:** Significant Improvement

Vignette based on actual lens falloff models.

**Falloff:**
```
cos4_falloff = pow(cos(angle_from_center), 4)
mechanical_vignette = smooth_circular_mask(uv, inner_radius, outer_radius)
final_vignette = cos4_falloff * mechanical_vignette
```

---

## 109. Panini Projection (PP)
**Novelty Class:** Significant Improvement

Wide FOV with reduced peripheral distortion.

**Projection:**
```
d = panini_distance_parameter  // 0-1
S = (d + 1) / (d + sqrt(x² + 1))
x' = S * x
y' = S * y / sqrt(x² + 1)
```

---

## 110. Color Blindness Simulation (CBS)
**Novelty Class:** Significant Improvement

Accurate simulation for accessibility testing.

**Simulation Matrices:**
```
protanopia_matrix = [...] 
deuteranopia_matrix = [...]
tritanopia_matrix = [...]
simulated = mul(input_color, deficiency_matrix)
```

---

## 111. Dithered Gradient Banding Fix (DGBF)
**Novelty Class:** Significant Improvement

Perceptually optimized dithering for gradient banding.

**Dithering:**
```
noise = triangular_pdf_noise(uv)  // Better than uniform
noise_scaled = noise * (1.0 / 255.0)  // 8-bit precision
result = input + noise_scaled
```

---

## 112. Screen-Space Reflections Enhancement (SSRE)
**Novelty Class:** Significant Improvement

Multi-step SSR with roughness-aware cone tracing.

**Enhancement:**
```
1. Hi-Z trace for initial hit
2. Binary search refinement
3. Roughness-dependent ray spread
4. Temporal accumulation of multiple rays
5. Fallback blending with cubemap
```

---

## 113. Exposure Adaptation with Histogram (EAH)
**Novelty Class:** Significant Improvement

Histogram-based auto-exposure with weighted zones.

**Adaptation:**
```
histogram = compute_weighted_histogram(input, zone_weights)
target_exposure = find_percentile(histogram, target_percentile)
current_exposure = lerp(current_exposure, target_exposure, adaptation_rate)
```

---

## 114. Local Contrast Enhancement (LCE)
**Novelty Class:** Novel Combination

CLAHE-inspired local contrast for games.

**Algorithm:**
```
1. Downsample to tile grid
2. Compute local histogram per tile
3. Clip and redistribute histogram
4. Compute local mapping curves
5. Apply with bilinear tile interpolation
```

---

## 115. Photographic Tonemapping Operators (PTO)
**Novelty Class:** Significant Improvement

Suite of film emulation curves.

**Operators:**
```
Kodachrome: saturated, warm highlights, cool shadows
Fuji Velvia: extreme saturation, high contrast
Cinestill 800T: tungsten balanced, halation
Portra 400: natural skin tones, soft rolloff
```

---

## 116. Motion Blur Quality Modes (MBQM)
**Novelty Class:** Significant Improvement

Scalable motion blur from mobile to high-end.

**Modes:**
```
Mobile: 4 samples, no scattering
Console: 8 samples, foreground separation
PC Quality: 16 samples, full scattering
Ultra: 32 samples, per-object blur
```

---

## 117. Screen-Space Contact Shadows (SSCS)
**Novelty Class:** Significant Improvement

Short-range ray marched shadows for contact detail.

**Ray March:**
```
for i in range(8):
    march_pos = surface_pos + light_dir * (i * step_size)
    depth_sample = sample_depth(project(march_pos))
    if depth_sample < march_pos.z:
        shadow += (1 - i/8)  // Soft falloff
```

---

## 118. Subsurface Scattering Screen-Space (SSSS)
**Novelty Class:** Significant Improvement

Separable bilateral filter for real-time SSS.

**Filter:**
```
// Horizontal pass
for i in range(-kernel_size, kernel_size):
    weight = sss_profile(i * step_size) * bilateral_weight(i)
    result += sample(uv + vec2(i * texel_size.x, 0)) * weight

// Vertical pass (same pattern)
```

---

## 119. Temporal Anti-Aliasing Sharpening (TAAS)
**Novelty Class:** Significant Improvement

Integrated sharpening within TAA to counter blur.

**Integration:**
```
resolved = taa_resolve(current, history)
sharpened = resolved + (resolved - blur(resolved)) * sharpening_strength
// Apply before output, not after
```

---

## 120. Screen-Space Global Illumination (SSGI)
**Novelty Class:** Significant Improvement

Single-bounce GI from screen-space ray marching.

**Algorithm:**
```
1. Generate sample directions (cosine-weighted hemisphere)
2. Ray march in screen space using Hi-Z
3. On hit: sample diffuse lighting from hit surface
4. Weight by BRDF and geometric term
5. Temporal accumulation for noise reduction
```

---

## 121. Geometric Antialiasing (GAA)
**Novelty Class:** Novel Combination

Detects and smooths geometric edges specifically.

**Detection:**
```
edge_score = detect_geometric_edges(depth, normal)
texture_edge_score = detect_texture_edges(color)
geometric_only = edge_score * (1 - texture_edge_score)
apply_smoothing(geometric_only)
```

---

## 122. HDR Bloom Conservation (HBC)
**Novelty Class:** Significant Improvement

Energy-conserving bloom that maintains overall brightness.

**Conservation:**
```
bloom = generate_bloom(hdr_input)
energy_bloom = sum(bloom) / pixel_count
energy_original = sum(hdr_input) / pixel_count
bloom_normalized = bloom * (energy_original / (energy_original + energy_bloom))
```

---

## 123. Bokeh Transparency Handling (BTH)
**Novelty Class:** Significant Improvement

Correct DoF for semi-transparent surfaces.

**Handling:**
```
1. Render transparent objects to separate buffer with depth
2. Compute CoC per transparent pixel
3. Scatter transparent bokeh separately
4. Composite transparent bokeh under opaque bokeh
```

---

## 124. Procedural Dirty Lens (PDL)
**Novelty Class:** Significant Improvement

Dynamic lens dirt based on environment.

**Procedural:**
```
base_dirt = sample_dirt_texture(uv)
rain_drops = animated_rain_simulation(uv, time)
smudges = perlin_noise(uv * smudge_scale)
combined_dirt = base_dirt + rain_drops * wet_factor + smudges
apply_to_bloom(combined_dirt)
```

---

## 125. Purkinje Effect Simulation (PES)
**Novelty Class:** Novel Combination

Shifts color perception at low light levels.

**Simulation:**
```
luminance = get_scene_luminance()
if luminance < mesopic_threshold:
    purkinje_shift = lerp(0, max_shift, 1 - luminance / mesopic_threshold)
    hue_shift(result, toward_blue, purkinje_shift)
    saturation_reduce(result, purkinje_shift * 0.5)
```

---

## 126. ACES Output Transform (AOT)
**Novelty Class:** Significant Improvement

Complete ACES pipeline for consistent HDR output.

**Pipeline:**
```
1. Input: scene-referred linear RGB
2. Apply Reference Rendering Transform (RRT)
3. Apply Output Device Transform (ODT) for target display
4. Support: sRGB, Rec.709, Rec.2020, DCI-P3, HDR10, Dolby Vision
```

---

## 127. Localized Tonemapping (LT)
**Novelty Class:** Novel Combination

Per-region tonemapping for HDR images.

**Algorithm:**
```
1. Segment image into luminance regions
2. Compute local tonemapping curve per region
3. Blend curves at region boundaries
4. Apply localized curves
```

---

## 128. Screen-Space Light Shafts (SSLS)
**Novelty Class:** Significant Improvement

Radial blur from occluded light sources.

**Algorithm:**
```
light_screen_pos = project(light_world_pos)
for each pixel:
    march_dir = normalize(uv - light_screen_pos)
    accumulated = 0
    for i in range(samples):
        sample_pos = uv - march_dir * i * step_size
        occluded = depth_test(sample_pos, light_depth)
        accumulated += occluded ? 0 : falloff(i)
    result += accumulated * light_color
```

---

## 129. Color Fringing (CF)
**Novelty Class:** Significant Improvement

Wavelength-dependent lateral chromatic aberration.

**Fringing:**
```
red_offset = chromatic_offset * 1.2
green_offset = chromatic_offset * 0.0
blue_offset = chromatic_offset * -0.8
result.r = sample(uv + radial_dir * red_offset).r
result.g = sample(uv + radial_dir * green_offset).g
result.b = sample(uv + radial_dir * blue_offset).b
```

---

## 130. Adaptive Sharpening (AS)
**Novelty Class:** Significant Improvement

Content-aware sharpening that avoids noise amplification.

**Adaptation:**
```
local_variance = compute_variance(neighborhood)
noise_threshold = estimate_noise_floor()
sharpening_amount = base_amount * saturate(1 - local_variance / noise_threshold)
result = sharpen(input, sharpening_amount)
```

---

# CATEGORY V: MATERIAL & BRDF SYSTEMS (Techniques 131-160)

## 131. Anisotropic GGX with Rotation (AGXR)
**Novelty Class:** Significant Improvement

Per-pixel anisotropy rotation from tangent map.

**BRDF:**
```
roughness_x = roughness * (1 + anisotropy)
roughness_y = roughness * (1 - anisotropy)
tangent_rotated = rotate(tangent, anisotropy_rotation)
D = GGX_anisotropic(H, N, tangent_rotated, roughness_x, roughness_y)
```

---

## 132. Multi-Layer Material System (MLMS)
**Novelty Class:** Patent-Worthy Invention

Arbitrary layer stacking with energy conservation.

**Evaluation:**
```
for each layer top-to-bottom:
    fresnel = schlick(layer.ior, VdotH)
    reflected = layer.brdf.eval() * fresnel
    transmitted = (1 - fresnel) * transmittance(layer.thickness)
    result += transmitted * accumulated_transmittance * reflected
    accumulated_transmittance *= transmitted
```

---

## 133. Iridescence BRDF (IBRDF)
**Novelty Class:** Patent-Worthy Invention

Thin-film interference for soap bubbles, oil slicks, beetle shells.

**Interference:**
```
optical_path_diff = 2 * thickness * refractive_index * cos(refracted_angle)
for wavelength in visible_spectrum:
    phase_diff = 2π * optical_path_diff / wavelength
    intensity = 2 * (1 + cos(phase_diff))
    result += spectral_to_rgb(wavelength) * intensity
```

---

## 134. Sheen BRDF for Fabric (SBF)
**Novelty Class:** Significant Improvement

Charlie distribution for velvet, cloth, fuzzy surfaces.

**Distribution:**
```
α = sheen_roughness²
D_charlie = (2 + 1/α) / (2π) * pow(sin(θ), 1/α)
F_sheen = sheen_color * fresnel_schlick(VdotH, 0.04)
sheen_brdf = D_charlie * F_sheen / (4 * NdotL * NdotV)
```

---

## 135. Clear Coat Layer (CCL)
**Novelty Class:** Significant Improvement

Automotive paint clear coat with proper layering.

**Layered BRDF:**
```
clear_coat_fresnel = schlick(1.5, VdotH)
clear_coat_D = GGX(H, N, clear_coat_roughness)
clear_coat = clear_coat_D * clear_coat_fresnel

base_attenuation = (1 - clear_coat_fresnel) * absorption(coat_thickness, base_to_coat)
base_layer = base_brdf.eval() * base_attenuation

result = clear_coat + base_layer
```

---

## 136. Microfiber BRDF (MBRDF)
**Novelty Class:** Patent-Worthy Invention

Explicit fiber scattering for realistic cloth.

**Fiber Model:**
```
fiber_tangent = sample_fiber_distribution(uv, fiber_density)
longitudinal = cos²((θ_in + θ_out) / 2)
azimuthal = cos(φ_out - φ_in + phase_shift)
fiber_brdf = longitudinal * azimuthal * fiber_color
```

---

## 137. Glitter BRDF (GBRDF)
**Novelty Class:** Patent-Worthy Invention

Discrete microflake reflections for sparkle effects.

**Sparkle:**
```
flake_normal = hash(uv * flake_density) * 2 - 1
flake_reflect = reflect(V, flake_normal)
sparkle = pow(max(0, dot(flake_reflect, L)), flake_sharpness)
sparkle *= step(random(), flake_coverage)
result = base_brdf + sparkle * flake_color
```

---

## 138. Subsurface Approximation (SA)
**Novelty Class:** Significant Improvement

Fast SSS for thin objects without blur pass.

**Approximation:**
```
thickness = sample_thickness_map(uv)
direct = standard_brdf(L, V, N)
wrapped = saturate(dot(N, L) * 0.5 + 0.5)
backscatter = wrapped * translucency * exp(-thickness / scatter_distance)
result = direct + backscatter * subsurface_color
```

---

## 139. Metallic Fresnel (MF)
**Novelty Class:** Significant Improvement

Conductor Fresnel with complex IOR.

**Complex Fresnel:**
```
// Use F82-tint model for artistic control
F_0 = base_color (for metals)
F_82 = lerp(F_0, tint_at_82_degrees, metallic_tint)
fresnel = F_0 + (1 - F_0) * pow5(1 - VdotH) * F_82_term
```

---

## 140. Wet Surface Modification (WSM)
**Novelty Class:** Significant Improvement

Physically-based surface wetness.

**Modification:**
```
wet_albedo = albedo * lerp(1.0, 0.2, wetness)  // Darker when wet
wet_roughness = roughness * lerp(1.0, 0.5, wetness)  // Smoother when wet
wet_metallic = metallic  // Unchanged
wet_F0 = lerp(F0, 0.02, wetness)  // Water layer reflection
```

---

## 141. Parallax Occlusion Mapping (POM)
**Novelty Class:** Significant Improvement

Self-shadowing and contact refinement.

**Algorithm:**
```
1. Linear search along view direction
2. Binary refinement at intersection
3. Shadow ray march to light
4. Self-shadow term from shadow ray hits
5. Contact shadow at silhouette
```

---

## 142. Detail Normal Blending (DNB)
**Novelty Class:** Significant Improvement

Correct normal map blending using UDN.

**Blending:**
```
// Unpack to [-1, 1]
base = base_normal * 2 - 1
detail = detail_normal * 2 - 1
// UDN blend
result.xy = base.xy + detail.xy
result.z = base.z
result = normalize(result)
```

---

## 143. Triplanar Mapping (TM)
**Novelty Class:** Significant Improvement

Seamless texturing for arbitrary geometry.

**Mapping:**
```
blend = pow(abs(normal), triplanar_sharpness)
blend /= dot(blend, vec3(1))
sample_x = texture(world_pos.yz)
sample_y = texture(world_pos.xz)
sample_z = texture(world_pos.xy)
result = sample_x * blend.x + sample_y * blend.y + sample_z * blend.z
```

---

## 144. Energy Compensating Multi-Scatter (ECMS)
**Novelty Class:** Significant Improvement

Kulla-Conty energy compensation.

**Compensation:**
```
E = directional_albedo_lut(NdotV, roughness)
E_avg = average_albedo_lut(roughness)
F_avg = (F0 + 1) / 2  // Average fresnel
f_ms = (1 - E) * (1 - E_avg) * F_avg * F_avg * E_avg / (1 - E_avg * (1 - F_avg))
result = (f_s / E + f_ms)  // Compensated BRDF
```

---

## 145. Bent Normal for AO (BNAO)
**Novelty Class:** Significant Improvement

Uses bent normal for diffuse, geometric normal for specular.

**Application:**
```
diffuse_irradiance = sample_environment(bent_normal) * ao
specular_irradiance = sample_environment(reflect(V, geometric_normal))
result = diffuse * diffuse_irradiance + specular * specular_irradiance
```

---

## 146. Height Blended Materials (HBM)
**Novelty Class:** Significant Improvement

Height-map driven material transitions.

**Blending:**
```
height_a = sample_height(uv, material_a)
height_b = sample_height(uv, material_b)
blend_factor = input_blend  // 0-1 from vertex color or mask

// Height-influenced blend
a_contribution = height_a + (1 - blend_factor)
b_contribution = height_b + blend_factor
blend_sharpness = 8.0
final_blend = saturate((b_contribution - a_contribution) * blend_sharpness + 0.5)
```

---

## 147. Procedural Wear and Aging (PWA)
**Novelty Class:** Patent-Worthy Invention

Curvature and AO-driven procedural wear.

**Wear Generation:**
```
curvature = compute_curvature(normal_map)
wear_mask = pow(curvature, wear_exponent)
wear_mask += (1 - ao) * ao_wear_influence
wear_mask += edge_detection(world_pos)
roughness += wear_mask * wear_roughness_increase
albedo = lerp(albedo, wear_color, wear_mask)
```

---

## 148. Toon Material with Ramp (TMR)
**Novelty Class:** Significant Improvement

Stylized shading with artist-controlled ramps.

**Ramp Shading:**
```
ndotl = dot(N, L) * 0.5 + 0.5
ramp_value = texture(ramp_texture, vec2(ndotl, 0)).r
diffuse = albedo * ramp_value * light_color
specular = step(threshold, ndoth) * specular_color  // Hard specular
rim = pow(1 - ndotv, rim_power) * step(rim_threshold, 1 - ndotv)
```

---

## 149. Holographic Material (HM)
**Novelty Class:** Patent-Worthy Invention

Diffractive holographic surfaces.

**Diffraction:**
```
grating_vector = sample_grating_texture(uv)
for wavelength in visible_spectrum:
    diffraction_angle = asin(grating_period / wavelength)
    intensity = grating_efficiency(diffraction_angle)
    diffracted_color += spectral_to_rgb(wavelength) * intensity
result = base_reflection + diffracted_color * hologram_intensity
```

---

## 150. Pearl BRDF (PBRDF)
**Novelty Class:** Patent-Worthy Invention

Multi-layer nacre interference.

**Nacre Model:**
```
for layer in range(num_layers):
    layer_thickness = base_thickness + layer * thickness_variation
    interference = thin_film_interference(layer_thickness, refractive_index)
    pearl_color += interference * layer_opacity
pearl_color /= num_layers
result = blend(base_reflection, pearl_color, view_angle)
```

---

## 151. Carbon Fiber Material (CFM)
**Novelty Class:** Significant Improvement

Weave pattern with anisotropic reflection.

**Weave:**
```
weave_pattern = sin(uv.x * frequency) * sin(uv.y * frequency)
fiber_direction = weave_pattern > 0 ? vec3(1,0,0) : vec3(0,1,0)
anisotropic_roughness = roughness * (1 + abs(weave_pattern) * anisotropy)
apply_anisotropic_ggx(fiber_direction, anisotropic_roughness)
clearcoat_layer(0.04, 0.1)  // Epoxy coating
```

---

## 152. Velvet BRDF (VBRDF)
**Novelty Class:** Significant Improvement

Rim-lit fuzzy surface scattering.

**Velvet Model:**
```
horizon_scatter = pow(1 - abs(ndotv), velvet_sharpness) * velvet_intensity
forward_scatter = pow(max(0, dot(V, L)), forward_power) * forward_intensity
result = albedo * (diffuse + horizon_scatter + forward_scatter)
```

---

## 153. Snow Material (SM)
**Novelty Class:** Significant Improvement

Crystalline subsurface scattering with sparkle.

**Snow Model:**
```
// Subsurface from ice crystals
sss = diffusion_profile_snow(thickness)
// Sparkle from crystal facets
sparkle = glitter_brdf(uv, crystal_density, light_dir)
// Fresnel from ice
fresnel = schlick(1.31, ndotv)
result = lerp(sss, specular, fresnel) + sparkle
```

---

## 154. Skin BRDF (SBRDF)
**Novelty Class:** Significant Improvement

Dual-specular lobe with SSS.

**Dual Lobe:**
```
primary_spec = ggx(roughness_primary) * 0.9
secondary_spec = ggx(roughness_secondary) * 0.1
combined_spec = primary_spec + secondary_spec

sss = separable_sss(diffuse_color)
result = combined_spec + sss
```

---

## 155. Eye Material (EM)
**Novelty Class:** Significant Improvement

Cornea refraction with iris parallax.

**Eye Model:**
```
cornea_refract = refract(V, cornea_normal, 1/1.376)
iris_uv = parallax_offset(base_uv, cornea_refract, iris_depth)
iris_color = texture(iris_texture, iris_uv)
limbal_darkening = pow(1 - ndotv, limbal_power)
caustic = caustic_pattern(light_dir, cornea_normal)
result = iris_color * limbal_darkening + cornea_spec + caustic
```

---

## 156. Hair BRDF (HBRDF)
**Novelty Class:** Patent-Worthy Invention

