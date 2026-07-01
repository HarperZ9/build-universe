<p align="center">
  <img src="docs/brand/build-universe-hero.png" alt="Build Universe: Buildlang Module & Example Surface">
</p>

# Build Universe v1.0.0

> A physics-inspired software ecosystem - language, OS kernel, graphics engines, trading systems, and AI frameworks, all written in BuildLang.

[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![buildlang](https://img.shields.io/badge/buildlang-.bld-orange.svg)
![version](https://img.shields.io/badge/version-1.0.0-informational.svg)
[![CI](https://github.com/HarperZ9/build-universe/actions/workflows/ci.yml/badge.svg)](https://github.com/HarperZ9/build-universe/actions/workflows/ci.yml)
![deps: none](https://img.shields.io/badge/deps-none-success.svg)
[![part of: Build ecosystem](https://img.shields.io/badge/part_of-Build_ecosystem-00b3a4.svg)](https://github.com/HarperZ9/build-universe)

A physics-inspired software ecosystem: programming language, operating system kernel, graphics engines, trading systems, and AI frameworks - all written in BuildLang.

## Modules

### Core
- **BuildLang** - Multi-paradigm systems language with algebraic effects, ownership, and a production C backend (HLSL/GLSL/LLVM/x86-64/ARM64/WASM/SPIR-V backends exist but are experimental and do not yet emit runnable artifacts)
- **BuildOS** - Hobby OS kernel (x86-64, ext2/4, context switching, memory management)
- **Axiom** - Neural architecture search and differentiable program synthesis

### Graphics
- **Photon** - Game rendering engine with shader injection and SPIR-V support
- **Spectrum** - Color science (ACES, Display P3, Rec.2020, spectral rendering)
- **Chromatic** - Perceptual color spaces (Oklab, JzAzBz, ICtCp, CAM16)
- **Lumina** - Post-processing pipeline
- **Nexus** - Universal mod framework
- **Prism** - Shader collection
- **Refract** - ENB integration
- **Neutrino** - Neural rendering effects

### Finance
- **Quantum Finance** - Algorithmic trading (momentum, mean reversion, stat arb)
- **Field Tensor** - 4D market data structure
- **Delta** - Options pricing and Greeks (Black-Scholes, binomial, Monte Carlo)
- **Entropy** - ML feature engineering and model training

### Integration
- **Entangle** - PC-mobile sync
- **Calibrate** - Display calibration
- **Nova** - Rendering presets

### Intelligence
- **Oracle** - Time-series forecasting (ARIMA, Holt-Winters, anomaly detection)
- **Wavelength** - Media processing

### Tools
- **Forge** - Developer tools (formatter, linter, debugger, profiler)
- **Foundation** - Standard library

## Status

**Alpha.** The BuildLang compiler (Rust; 755 test functions in tree) is the most mature component. The C backend produces correct native binaries. HLSL/GLSL produce clean shader output. Other backends are experimental. The .bld modules demonstrate the language's capabilities across domains. See [buildos/STATUS.md](buildos/STATUS.md) for kernel implementation state.

## Caveats

- **This ecosystem does not compile as a whole.** Each module depends on the BuildLang compiler (separate repo: [HarperZ9/buildlang](https://github.com/HarperZ9/buildlang)). The compiler can compile individual modules but cross-module resolution is not yet complete.
- **BuildOS** is an educational hobby kernel, not a production OS. See [buildos/STATUS.md](buildos/STATUS.md).
- **Axiom** is an experimental proof-of-concept for differentiable program synthesis.
- The `.bld` source files serve as both working code and language specification - demonstrating BuildLang's syntax across domains.

## Ground Truth

This repo previously carried conflicting claims across README, ENGINEERING, and CHANGELOG. Authoritative per-module reality now lives in:

- [STATUS.md](STATUS.md) - module maturity ledger (real vs scaffolding). Where any doc disagrees, STATUS.md is canonical.
- [LINEAGE.md](LINEAGE.md) - the Build family tree and how the mixed-language pieces interlace.
- [docs/HEATMAP-AND-ACTION-PLAN.md](docs/HEATMAP-AND-ACTION-PLAN.md) - engineering heatmap and prioritized plan.

## License

MIT License. See [LICENSE](LICENSE).
