# QUANTA UNIVERSE Production Implementation

## Project Context

You are expanding a 26K-line software specification into working implementations. The project comprises 22 modules across 6 categories.

## Repository Location

Clone or locate: `QUANTA-UNIVERSE/` containing:
- `/core/` — QuantaLang, QuantaOS, Axiom
- `/rendering/` — Photon, Spectrum, Chromatic, Lumina, Nexus, Prism, Refract, Neutrino
- `/trading/` — Quantum Finance, Field Tensor, Delta, Entropy
- `/integration/` — Entangle, Calibrate, Nova
- `/ai/` — Oracle, Wavelength
- `/tools/` — Forge, Foundation

## Implementation Priority Order

### Phase 1: Core Platform (Critical Path)
1. **QuantaLang** — Expand compiler:
   - Full lexer with Unicode support
   - Recursive descent parser with error recovery
   - Type inference engine (Hindley-Milner + extensions)
   - Multiple backends: LLVM IR, WASM, SPIR-V
   - Runtime with GC (tracing or reference counting)

2. **QuantaOS** — Bootable kernel:
   - UEFI bootloader (Rust, `uefi-rs` crate)
   - Physical/virtual memory management (buddy allocator + 4-level paging)
   - Process scheduler (implement Neural Process Scheduler from spec)
   - System call interface (500-series AI syscalls per spec)
   - VFS layer with initramfs support
   - Test environment: QEMU x86_64

3. **Foundation** — Standard library:
   - Collections (Vec, HashMap, BTreeMap, etc.)
   - I/O abstractions
   - Concurrency primitives (Mutex, Channel, async runtime)

### Phase 2: Rendering Stack
4. **Photon** — Graphics hook engine:
   - DirectX 11/12 hook implementation (detours-based)
   - Vulkan layer intercept
   - Shader bytecode injection framework

5. **Spectrum** — Color science:
   - All 12 tonemappers fully implemented with SIMD
   - Color space conversions (sRGB, DCI-P3, Rec.2020, ACES)
   - HDR pipeline

6. Remaining rendering modules: Chromatic, Lumina, Nexus, Prism, Refract, Neutrino

### Phase 3: Trading Systems
7. **Quantum Finance** — Trading engine:
   - Order management system
   - Risk management with position limits
   - Alpaca/IBKR broker integration

8. **Field Tensor** — Market data structure:
   - 4th-order tensor implementation
   - Real-time OHLCV ingestion

9. Delta, Entropy modules

### Phase 4: AI & Integration
10. Axiom, Oracle, Wavelength, Entangle, Calibrate, Nova

## Technical Standards

### Code Quality
- All functions documented with doc comments
- Unit test coverage >80%
- Integration tests for cross-module interfaces
- No `unsafe` blocks without `// SAFETY:` justification (Rust modules)
- Error handling: Result types, no panics in library code

### Architecture Patterns
- Dependency injection for testability
- Interface segregation (small, focused traits)
- Zero-cost abstractions where possible
- Memory: No allocations in hot paths

### File Structure Per Module
```
module_name/
├── Cargo.toml (or equivalent build config)
├── src/
│   ├── lib.quanta        # Public API
│   ├── internal/         # Private implementation
│   └── ffi/              # C ABI exports if needed
├── tests/
│   ├── unit/
│   └── integration/
├── benches/              # Performance benchmarks
└── README.md
```

### Commit Standards
- Atomic commits, one logical change per commit
- Format: `[module] verb: description`
- Examples:
  - `[quantaos] feat: implement buddy allocator`
  - `[spectrum] fix: clamp HDR values before tonemapping`
  - `[photon] refactor: extract hook registry to separate module`

## Current Session Instructions

Begin with whichever module the user specifies, or if unspecified, start with **QuantaLang** Phase 1 expansion:

1. Read existing `/core/quantalang/lib.quanta`
2. Create production directory structure
3. Implement lexer with full Unicode identifier support
4. Implement parser with Pratt parsing for expressions
5. Add comprehensive test suite
6. Commit incrementally

For each implementation session:
- State which module and component you're implementing
- Show key architectural decisions before coding
- Write tests alongside implementation
- Run tests before committing
- Report progress

## IP Preservation

Maintain all copyright headers:
```
// Copyright (c) 2024-2025 Zain Dana Harper. All Rights Reserved.
```



## Begin

Confirm you've read the existing codebase, then propose a detailed implementation plan for the first module.