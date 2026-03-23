# HARPER ENGINE — Reader's Guide
## Complete Navigation & Reference Manual

**Purpose:** This guide helps you efficiently navigate 150 whitepapers and 800 rendering techniques.

---

## TABLE OF CONTENTS

1. [How to Use This Archive](#how-to-use-this-archive)
2. [Recommended Reading Paths](#recommended-reading-paths)
3. [Complete Whitepaper Index](#complete-whitepaper-index)
4. [Complete Techniques Index](#complete-techniques-index)
5. [Patent-Worthy Innovations](#patent-worthy-innovations)
6. [Quick Reference Tables](#quick-reference-tables)

---

## HOW TO USE THIS ARCHIVE

### Document Types

| Type | Format | Location | Best For |
|------|--------|----------|----------|
| Whitepapers | .docx | `/whitepapers/` | Deep understanding, academic reference |
| Techniques | .md | `/techniques/` | Quick implementation, code snippets |

### Reading Approach

**For Learning:** Follow the recommended paths below
**For Implementation:** Jump directly to relevant technique numbers
**For Evaluation:** Review patent-worthy innovations section

---

## RECOMMENDED READING PATHS

### Path A: Engine Architecture (2-3 hours)
*Understand the complete rendering pipeline*

```
WP-001 → WP-002 → WP-031 → WP-082 → WP-109 → WP-150
```

| Order | Paper | Topic |
|-------|-------|-------|
| 1 | WP-001 | State-of-the-Art Survey 2020-2025 |
| 2 | WP-002 | Innovation Gap Analysis |
| 3 | WP-031 | GPU-Driven Rendering Pipeline |
| 4 | WP-082 | GPU Work Graph Rendering |
| 5 | WP-109 | Work Graphs for Rendering |
| 6 | WP-150 | Hybrid Rendering Pipeline |

---

### Path B: Ray Tracing Mastery (3-4 hours)
*Complete ray tracing implementation*

```
WP-051 → WP-052 → WP-053 → WP-102 → WP-110 → WP-123 → WP-149
```

| Order | Paper | Topic |
|-------|-------|-------|
| 1 | WP-051 | Wavefront Path Regeneration |
| 2 | WP-052 | Cone-Traced Reflections |
| 3 | WP-053 | Persistent Thread Pools |
| 4 | WP-102 | Stochastic Light Trees |
| 5 | WP-110 | Neural Radiance Caching |
| 6 | WP-123 | Ray Tracing Denoising |
| 7 | WP-149 | Ray-Traced Ambient Occlusion |

---

### Path C: Neural Rendering (2-3 hours)
*Machine learning for graphics*

```
WP-055 → WP-059 → WP-060 → WP-101 → WP-105 → WP-115
```

| Order | Paper | Topic |
|-------|-------|-------|
| 1 | WP-055 | Cooperative Matrix Neural Inference |
| 2 | WP-059 | Gaussian Splat Compression |
| 3 | WP-060 | Neural Subsurface Scattering |
| 4 | WP-101 | Neural Geometry Compression |
| 5 | WP-105 | Gaussian Splatting Animation |
| 6 | WP-115 | Neural BRDF Compression |

---

### Path D: Production Effects (3-4 hours)
*Ship-ready visual effects*

```
WP-091 → WP-130 → WP-131 → WP-139 → WP-138 → WP-144
```

| Order | Paper | Topic |
|-------|-------|-------|
| 1 | WP-091 | Cinematic Depth of Field |
| 2 | WP-130 | Bokeh Depth of Field |
| 3 | WP-131 | Motion Blur |
| 4 | WP-139 | Cloud Rendering |
| 5 | WP-138 | Water Rendering |
| 6 | WP-144 | Screen-Space Reflections |

---

### Path E: Global Illumination (2-3 hours)
*Indirect lighting solutions*

```
WP-062 → WP-071 → WP-120 → WP-147 → WP-148 → WP-110
```

| Order | Paper | Topic |
|-------|-------|-------|
| 1 | WP-062 | Bent Cone Ambient Occlusion |
| 2 | WP-071 | Anisotropic Irradiance Probes |
| 3 | WP-120 | Surfel-Based Global Illumination |
| 4 | WP-147 | Voxel Global Illumination |
| 5 | WP-148 | Light Propagation Volumes |
| 6 | WP-110 | Neural Radiance Caching |

---

### Path F: Material Systems (2 hours)
*PBR and advanced materials*

```
WP-080 → WP-081 → WP-083 → WP-136 → WP-137 → WP-135
```

| Order | Paper | Topic |
|-------|-------|-------|
| 1 | WP-080 | Multi-Layer Material System |
| 2 | WP-081 | Iridescence BRDF |
| 3 | WP-083 | Hair BRDF |
| 4 | WP-136 | Skin Rendering |
| 5 | WP-137 | Fabric Rendering |
| 6 | WP-135 | Eye Rendering |

---

## COMPLETE WHITEPAPER INDEX

### Volume 1: WP-001 to WP-050

| ID | Title | Category |
|----|-------|----------|
| 001 | Real-Time Rendering State of the Art 2020-2025 | Survey |
| 002 | Innovation Gap Analysis: 47 Opportunities | Analysis |
| 003 | Temporal Anti-Aliasing: A Comprehensive Framework | AA |
| 004 | Visibility Buffer Rendering Architecture | Pipeline |
| 005 | Meshlet-Based Geometry Processing | Geometry |
| 006 | Blue Noise Dithering for Real-Time Applications | Sampling |
| 007 | Stochastic Screen-Space Reflections | Reflections |
| 008 | Hierarchical Z-Buffer Occlusion Culling | Culling |
| 009 | Physically-Based Atmospheric Scattering | Atmosphere |
| 010 | Adaptive Shadow Map Resolution | Shadows |
| 011 | ReSTIR: Reservoir-Based Spatiotemporal Importance Resampling | RT |
| 012 | Micro-Facet BRDF Theory and Implementation | Materials |
| 013 | Screen-Space Global Illumination | GI |
| 014 | Real-Time Subsurface Scattering | SSS |
| 015 | Volumetric Fog and Lighting | Volumetrics |
| 016 | GPU-Driven Indirect Rendering | Pipeline |
| 017 | Order-Independent Transparency | Transparency |
| 018 | Texture Space Shading | Optimization |
| 019 | Variable Rate Shading Strategies | Optimization |
| 020 | Nanite-Inspired Geometry Virtualization | Geometry |
| 021-050 | [Additional foundational papers] | Various |

### Volume 2: WP-051 to WP-100

| ID | Title | Category |
|----|-------|----------|
| 051 | Wavefront Path Regeneration | RT |
| 052 | Cone-Traced Reflections | Reflections |
| 053 | Persistent Thread Pools | Compute |
| 054 | Mesh Shader Particle Systems | Particles |
| 055 | Cooperative Matrix Neural Inference | Neural |
| 056 | Virtual Shadow Maps | Shadows |
| 057 | Temporal Gradient Domain Filtering | Denoising |
| 058 | TITAN Color Pipeline | Color |
| 059 | Gaussian Splat Compression | Neural |
| 060 | Neural Subsurface Scattering | Neural |
| 061 | Stochastic Light Clustering | Lighting |
| 062 | Bent Cone Ambient Occlusion | AO |
| 063 | Differentiable Rasterization Bridge | ML |
| 064 | Contact Hardening Shadows | Shadows |
| 065 | Film Emulation System | Post |
| 066 | Procedural Aurora Borealis | Effects |
| 067 | Underwater Caustics Volume | Water |
| 068 | Real-Time Fluid Surface Reconstruction | Fluids |
| 069 | Perceptual Gamut Mapping | Color |
| 070 | Neural Level of Detail | Neural |
| 071-100 | [Additional advanced papers] | Various |

### Volume 3: WP-101 to WP-150

| ID | Title | Category |
|----|-------|----------|
| 101 | Neural Geometry Compression | Neural |
| 102 | Stochastic Light Trees | Lighting |
| 103 | Froxel-Based Volumetric Lighting | Volumetrics |
| 104 | Differentiable Rasterization | ML |
| 105 | Gaussian Splatting Animation | Neural |
| 106 | Temporal Gradient-Domain Rendering | Denoising |
| 107 | Multi-Layer Material System | Materials |
| 108 | Virtual Shadow Maps | Shadows |
| 109 | Work Graphs for Rendering | Pipeline |
| 110 | Neural Radiance Caching | Neural/GI |
| 111-125 | [Lighting & Shading Systems] | Various |
| 126-142 | [Atmospheric & Environmental] | Various |
| 143-150 | [Effects & Integration] | Various |

---

## COMPLETE TECHNIQUES INDEX

### Volume 1: Techniques 001-300

| Range | Category | Key Techniques |
|-------|----------|----------------|
| 001-030 | Shading Models | PBR, Disney BRDF, Subsurface |
| 031-060 | Shadow Techniques | CSM, VSM, PCSS, Contact |
| 061-090 | Anti-Aliasing | TAA, FXAA, SMAA, MSAA |
| 091-120 | Material Systems | Multi-layer, Anisotropic, Cloth |
| 121-150 | Screen-Space Effects | SSR, SSAO, SSGI |
| 151-180 | Transparency | OIT, Alpha-to-Coverage |
| 181-210 | Post-Processing | Bloom, DoF, Motion Blur |
| 211-240 | Culling & LOD | Hi-Z, Occlusion, Meshlets |
| 241-270 | Texture Techniques | Streaming, Compression, Filtering |
| 271-300 | Optimization | Wave Intrinsics, Occupancy |

### Volume 2: Techniques 301-600

| Range | Category | Key Techniques |
|-------|----------|----------------|
| 301-335 | Advanced Ray Tracing | Wavefront, ReSTIR, BVH |
| 336-370 | Compute Shaders | Wave Ops, Barriers, Dispatch |
| 371-405 | Geometry Processing | Mesh Shaders, Tessellation |
| 406-440 | Texture & Sampling | Virtual Textures, Bombing |
| 441-480 | Lighting & Shadows | Clustered, Area Lights, LTC |
| 481-520 | Post-Processing Advanced | TSR, Neural, HDR |
| 521-560 | Temporal Techniques | Reprojection, Accumulation |
| 561-600 | Integration | Pipeline, Memory, Profiling |

### Volume 3: Techniques 601-800

| Range | Category | Key Techniques |
|-------|----------|----------------|
| 601-630 | Neural Rendering | NGC, DVS, NTS, NAO |
| 631-670 | Advanced Geometry | NVG, CMC, GPU Simplification |
| 671-720 | Advanced Lighting | SLT, WSIC, PMA, MLMC |
| 721-760 | Temporal Systems | EHB, VWR, DD, TSS |
| 761-800 | Volumetric & Atmospheric | Froxels, Clouds, God Rays |

---

## PATENT-WORTHY INNOVATIONS

The following represent the most novel contributions suitable for patent filing:

### Tier 1: Foundational Inventions (Highest Value)

| ID | Title | Innovation |
|----|-------|------------|
| WP-051 | Wavefront Path Regeneration | Dynamic SIMD lane refill architecture |
| WP-053 | Persistent Thread Pools | GPU work-stealing with termination detection |
| WP-054 | Mesh Shader Particle Systems | Vertex-buffer-free particle expansion |
| WP-055 | Cooperative Matrix Neural Inference | Tensor cores in shader pipelines |
| WP-082 | GPU Work Graph Rendering | Self-scheduling render pipeline |
| WP-101 | Neural Geometry Compression | VAE-based mesh encoding |
| WP-102 | Stochastic Light Trees | O(log N) light importance sampling |

### Tier 2: Significant Inventions (High Value)

| ID | Title | Innovation |
|----|-------|------------|
| WP-056 | Virtual Shadow Maps | Page-based shadow virtualization |
| WP-059 | Gaussian Splat Compression | Learned vector quantization for 3DGS |
| WP-074 | Gaussian Animation System | Skeletal deformation for Gaussians |
| WP-105 | Gaussian Splatting Animation | Production-ready animated 3DGS |
| WP-109 | Work Graphs for Rendering | DX12 Ultimate integration |
| WP-110 | Neural Radiance Caching | Learned indirect illumination |

### Tier 3: Novel Techniques (Standard Value)

| Count | Category |
|-------|----------|
| 45 | Ray tracing optimizations |
| 38 | Neural rendering techniques |
| 32 | Geometry processing methods |
| 28 | Temporal algorithms |
| 22 | Material innovations |

**Total Patent-Worthy Items:** ~165

---

## QUICK REFERENCE TABLES

### Performance Targets (RTX 4090, 1440p)

| Effect | Target | Paper Reference |
|--------|--------|-----------------|
| Ray-traced shadows | < 1ms | WP-056, WP-064 |
| Global illumination | < 2ms | WP-110, WP-147 |
| Volumetric lighting | < 1.5ms | WP-103 |
| TAA + upscaling | < 0.5ms | WP-003, WP-079 |
| Denoising | < 0.3ms | WP-057, WP-123 |

### Memory Budgets

| System | Budget | Paper Reference |
|--------|--------|-----------------|
| Shadow maps | 64MB | WP-056, WP-108 |
| GI probes | 32MB | WP-071, WP-120 |
| Temporal buffers | 48MB | WP-003, WP-097 |
| Light data | 16MB | WP-102, WP-061 |

### Shader Model Requirements

| Feature | Minimum SM | Recommended SM |
|---------|------------|----------------|
| Basic techniques | 6.0 | 6.5 |
| Mesh shaders | 6.5 | 6.6 |
| Work graphs | 6.8 | 6.8 |
| Neural inference | 6.5 | 6.6 |

---

## CROSS-REFERENCE MATRIX

Find related papers by topic:

| If you're reading... | Also read... |
|---------------------|--------------|
| WP-051 (Wavefront Path Regen) | WP-053, WP-102, WP-110 |
| WP-054 (Mesh Shader Particles) | WP-082, WP-124, WP-142 |
| WP-055 (Cooperative Matrix) | WP-101, WP-115, WP-110 |
| WP-103 (Froxel Volumetrics) | WP-139, WP-077, WP-084 |
| WP-108 (Virtual Shadow Maps) | WP-056, WP-064, WP-093 |

---

## IMPLEMENTATION PRIORITY

For a new engine, implement in this order:

### Phase 1: Foundation (Month 1-2)
1. Visibility buffer (WP-004, Techniques 211-220)
2. Clustered lighting (WP-111, Techniques 441-450)
3. Basic TAA (WP-003, Techniques 061-070)
4. Shadow maps (WP-010, Techniques 031-040)

### Phase 2: Quality (Month 3-4)
1. Screen-space effects (Techniques 121-150)
2. PBR materials (WP-012, Techniques 001-030)
3. Post-processing (Techniques 181-210)
4. Volumetrics (WP-103, Techniques 761-780)

### Phase 3: Advanced (Month 5-6)
1. Ray tracing integration (WP-051, WP-102)
2. Neural techniques (WP-055, WP-110)
3. Work graphs (WP-082, WP-109)
4. Advanced materials (WP-080, WP-107)

---

*Harper Engine — Complete Technical Reference*
*© 2025 Harper Research Division*
