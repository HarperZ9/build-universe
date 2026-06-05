# QUANTA-UNIVERSE — Module Maturity Ledger (Canonical)

Last verified: 2026-06-05. This file is the single source of truth for module
reality. Where README, ENGINEERING, CHANGELOG, UNIVERSE.toml, or CATALOG.json
disagree, this file wins. Scores are engineering concreteness (0-10) from direct
source audit: real, correct, extractable logic vs scaffolding/showcase.

## Canonical facts

- Version: 1.0.0  |  License: MIT (see LICENSE)
- Compiler: 755 test functions in tree; only the C backend is end-to-end.
  HLSL/GLSL emit text; x86-64/ARM64/WASM/LLVM/SPIR-V emit output but have no
  linker/assembler integration (no runnable artifacts yet).
- Each .quanta module transpiles to C individually. Whole-ecosystem cross-module
  compilation and compiler self-hosting are not yet achieved.
- ~6 GB of local disk is Cargo target/ build cache (already git-ignored; the
  compiler dir quantalang/ is itself git-ignored, a separate repo).

## Tier 1 — Load-bearing, real, extractable (the engine)

| Component | Score | What is real |
|---|---|---|
| quantalang compiler (C backend) | 6.5 | Full front-to-C pipeline; monomorphization, traits/vtables, one-shot effects; 755 test fns |
| programs/ (56 MSVC exes) | 9.0 | qdb (SQL), qparse, qsed, grep, base64, calc, color_test 12/12 |
| spectrum (color science) | 9.0 | sRGB/XYZ matrices, PQ/HLG EOTFs, 13 tonemappers, verified OKLab constants |
| chromatic (perceptual color) | 8.0 | LAB/RGB with matrix inversion, gamut mapping via binary search |
| delta (options pricing) | 8.0 | Black-Scholes exact, full Greeks, Newton-Raphson + Brent IV |
| foundation math / crypto | 8.0 / 7.0 | SHA-256 FIPS 180-4 correct; trig/pow/log via intrinsics |

## Tier 2 — Real kernel inside scaffolding (extract the core)

| Component | Score | Real core | Scaffolding |
|---|---|---|---|
| quantaos kernel | 6.5 | Memory, scheduler, ext2/4, IPC, drivers, TCP/IP stack | AI syscalls return -1; self-healing is Z-score, not ML |
| entropy | 7.0 | LSTM forward pass; GBDT variance splits | No backprop |
| oracle | 7.0 | SARIMA differencing + AR/MA fitting | Forecast integration thin |
| field-tensor | 6.0 | Cholesky, power-iteration eigenvalues, indicators | 4D market application sparse |
| quantum-finance | 6.0 | OHLCV, TWAP/VWAP, risk calcs | Broker APIs return empty data |
| photon | 6.0 | ~15% real: meshlet culling, attenuation, ray differentials | ~85% boilerplate; hooks return false |
| prism | 7.0 | Correct HLSL tonemapper math | No GPU compile/injection |
| axiom | 5.0 | Forward-mode dual-number autodiff correct | Mutation ops stubbed; MAML/CMA-ES sketched |
| foundation collections | 6.0 | Vec correct | HashMap/BTreeMap sparse; regex executor missing |

## Tier 3 — Sketch / showcase (design intent, not engineering)

These remain in-tree (interlaced into build/interconnect wiring) but are not to
be represented as implemented engineering.

| Component | Score | Reality |
|---|---|---|
| lumina | 5.0 | Preset configs; claimed FFT bloom has no FFT code |
| refract | 5.0 | ENB metadata; no D3D11 hook installation |
| forge | 5.0 | Logger real; linter/debugger/profiler are shells |
| neutrino | 4.0 | Neural rendering is type enums; zero tensor ops |
| nexus | 4.0 | Mod-framework skeleton; no loader/solver |
| wavelength | 4.0 | Media containers; no DSP/codec |

## Infrastructure dirs (not domain modules)

cli, config, runtime, repl, lsp, fmt, debug, test, profiler, bench, benchmarks,
pkg, docs, universe, examples are scaffolding/glue for the ecosystem manifest,
of mixed completeness. Treat as supporting tooling, not headline modules.
