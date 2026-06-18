# Quanta Universe v1.0.0

> A physics-inspired software ecosystem - language, OS kernel, graphics engines, trading systems, and AI frameworks, all written in QuantaLang.

[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![quantalang](https://img.shields.io/badge/quantalang-.quanta-orange.svg)
![version](https://img.shields.io/badge/version-1.0.0-informational.svg)
[![CI](https://github.com/HarperZ9/quanta-universe/actions/workflows/ci.yml/badge.svg)](https://github.com/HarperZ9/quanta-universe/actions/workflows/ci.yml)
![deps: none](https://img.shields.io/badge/deps-none-success.svg)
[![part of: Quanta ecosystem](https://img.shields.io/badge/part_of-Quanta_ecosystem-00b3a4.svg)](https://github.com/HarperZ9/quanta-universe)

A physics-inspired software ecosystem: programming language, operating system kernel, graphics engines, trading systems, and AI frameworks - all written in QuantaLang.

## Modules

### Core
- **QuantaLang** - Multi-paradigm systems language with algebraic effects, ownership, and a production C backend (HLSL/GLSL/LLVM/x86-64/ARM64/WASM/SPIR-V backends exist but are experimental and do not yet emit runnable artifacts)
- **QuantaOS** - Hobby OS kernel (x86-64, ext2/4, context switching, memory management)
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

**Alpha.** The QuantaLang compiler (Rust; 755 test functions in tree) is the most mature component. The C backend produces correct native binaries. HLSL/GLSL produce clean shader output. Other backends are experimental. The .quanta modules demonstrate the language's capabilities across domains. See [quantaos/STATUS.md](quantaos/STATUS.md) for kernel implementation state.

### Publication Map

Split-repo package metadata now lives in [tools/package-index.toml](tools/package-index.toml).
Module slugs follow lowercase-hyphen naming for public-facing package IDs.
Use [releases/release-candidates.md](releases/release-candidates.md) for the
current publish dashboard and [tools/showcase.md](tools/showcase.md) for the
audience-facing module surface.

## Usage

This is a multi-module ecosystem, not a single installable package. The two
things you actually run are the `quantac` compiler (transpile/build `.quanta`
modules and programs) and the Python organism tooling (`tools/verify_organism.py`,
`tools/release_plan.py`). See **[USAGE.md](USAGE.md)** for install/build lines,
the real commands, and worked examples. A runnable demo lives in
[examples/demo/](examples/demo/).

Quick orientation:

```sh
# 1. Verify which components actually build/pass on this machine (no compiler needed)
python tools/verify_organism.py --quick

# 2. Transpile a module or program to C with the quantac compiler (separate repo)
quantac programs/echo.quanta --target c -o /dev/null
```

`quantac` comes from the separate [HarperZ9/quantalang](https://github.com/HarperZ9/quantalang)
repo (build it with `cargo build --release`); it is not bundled here.

## Caveats

- **This ecosystem does not compile as a whole.** Each module depends on the QuantaLang compiler (separate repo: [HarperZ9/quantalang](https://github.com/HarperZ9/quantalang)). The compiler can compile individual modules but cross-module resolution is not yet complete.
- **QuantaOS** is an educational hobby kernel, not a production OS. See [quantaos/STATUS.md](quantaos/STATUS.md).
- **Axiom** is an experimental proof-of-concept for differentiable program synthesis.
- The `.quanta` source files serve as both working code and language specification - demonstrating QuantaLang's syntax across domains.

## Ground Truth

This repo previously carried conflicting claims across README, ENGINEERING, and CHANGELOG. Authoritative per-module reality now lives in:

- [STATUS.md](STATUS.md) - module maturity ledger (real vs scaffolding). Where any doc disagrees, STATUS.md is canonical.
- [LINEAGE.md](LINEAGE.md) - the Quanta family tree and how the mixed-language pieces interlace.
- [docs/HEATMAP-AND-ACTION-PLAN.md](docs/HEATMAP-AND-ACTION-PLAN.md) - engineering heatmap and prioritized plan.

## License

MIT License. See [LICENSE](LICENSE).
