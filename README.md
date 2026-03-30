# Quanta Universe v1.0.0

A physics-inspired software ecosystem: programming language, operating system kernel, graphics engines, trading systems, and AI frameworks — all written in QuantaLang.

## Modules

### Core
- **QuantaLang** — Multi-paradigm systems language with algebraic effects, ownership, and 8 codegen backends
- **QuantaOS** — Hobby OS kernel (x86-64, ext2/4, context switching, memory management)
- **Axiom** — Neural architecture search and differentiable program synthesis

### Graphics
- **Photon** — Game rendering engine with shader injection and SPIR-V support
- **Spectrum** — Color science (ACES, Display P3, Rec.2020, spectral rendering)
- **Chromatic** — Perceptual color spaces (Oklab, JzAzBz, ICtCp, CAM16)
- **Lumina** — Post-processing pipeline
- **Nexus** — Universal mod framework
- **Prism** — Shader collection
- **Refract** — ENB integration
- **Neutrino** — Neural rendering effects

### Finance
- **Quantum Finance** — Algorithmic trading (momentum, mean reversion, stat arb)
- **Field Tensor** — 4D market data structure
- **Delta** — Options pricing and Greeks (Black-Scholes, binomial, Monte Carlo)
- **Entropy** — ML feature engineering and model training

### Integration
- **Entangle** — PC-mobile sync
- **Calibrate** — Display calibration
- **Nova** — Rendering presets

### Intelligence
- **Oracle** — Time-series forecasting (ARIMA, Holt-Winters, anomaly detection)
- **Wavelength** — Media processing

### Tools
- **Forge** — Developer tools (formatter, linter, debugger, profiler)
- **Foundation** — Standard library

## Status

**Alpha.** The QuantaLang compiler (81K lines Rust, 599 tests passing) is the most mature component. The C backend produces correct native binaries. HLSL/GLSL produce clean shader output. Other backends are experimental. The .quanta modules demonstrate the language's capabilities across domains. See [quantaos/STATUS.md](quantaos/STATUS.md) for kernel implementation state.

## Caveats

- **This ecosystem does not compile as a whole.** Each module depends on the QuantaLang compiler (separate repo: [HarperZ9/quantalang](https://github.com/HarperZ9/quantalang)). The compiler can compile individual modules but cross-module resolution is not yet complete.
- **QuantaOS** is an educational hobby kernel, not a production OS. See [quantaos/STATUS.md](quantaos/STATUS.md).
- **Axiom** is an experimental proof-of-concept for differentiable program synthesis.
- The `.quanta` source files serve as both working code and language specification — demonstrating QuantaLang's syntax across domains.

## License

MIT License. See [LICENSE](LICENSE).
