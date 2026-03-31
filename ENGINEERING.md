# Quanta Ecosystem — Engineering Runbook

Last verified: 2026-03-31. All CI green, all claims backed by evidence.

## Quick Reference

| Repo | Tests | CI | Release |
|------|-------|----|---------|
| quantalang | 604 Rust | clippy + fmt + test + e2e color self-check | v1.0.0 (quantac.exe) |
| calibrate-pro | 297 Python | ruff + pytest (Ubuntu + Windows) | v1.0.0 (standalone exe) |
| quanta-color | 457 Python | ruff + pytest | v1.0.0 |
| quanta-universe | — | file validation | v1.0.0 |
| quanta-finance | 142 Python | ruff + pytest | — |
| quanta-oracle | 187 Python | ruff + pytest | — |
| quanta-engine | 96 Python | ruff + pytest (stubs for cross-deps) | — |
| quanta-ui | 17 Python | ruff + pytest (needs libEGL on Linux) | — |
| quanta-ecosystem | — | sdist build validation | — |
| aurora | Lua | LUA_PATH set, Lua 5.1 compat | — |

## When CI Fails

### quantalang
- **clippy fails**: Check `-D clippy::correctness`. Style/perf/complexity are allowed.
- **fmt fails**: Run `cargo fmt` locally, commit.
- **test fails**: Run `cargo test -- --skip spirv` locally. 3 SPIR-V tests are skipped (need spirv-val).
- **e2e compilation fails**: A .quanta program doesn't compile. Check if a type system change broke it.
- **color_test fails**: The self-verifying binary disagrees with CIE 1976. Check float codegen.

### calibrate-pro
- **quanta-color not found**: It's installed via `git+https://github.com/HarperZ9/quanta-color.git`. If quanta-color CI is red, calibrate-pro will fail too.
- **Windows-only test fails on Linux**: Tests using `ctypes.windll` are skipped on Linux. If new Windows tests are added, mark them `@pytest.mark.skipif(sys.platform != "win32")`.

### quanta-engine
- **Cross-project imports fail**: Tests that import `quanta_finance` or `quanta_oracle` are skipped in CI (private repos). Run locally with all packages installed.
- **Timing tests fail on Windows**: Rate-limiting tests are skipped on Windows CI.

### aurora
- **Lua module not found**: Ensure `LUA_PATH="./?.lua;./?/init.lua;;"` is set.
- **Lua 5.1 syntax error**: Don't use `or` as standalone expression statements. Use `if not ... then` chains.

## Adding a New .quanta Program

1. Write the program in `programs/`
2. Verify it compiles: `quantac your_program.quanta --target c -o /dev/null`
3. If it uses `pow()`, `sqrt()`, or float division — verify `1.0/3.0` produces `0.333`, not `0`
4. If it calls methods defined later in the same impl block — move the callee above the caller
5. Don't use `&str` as function parameters (codegen bug: value vs pointer)
6. Commit and push — CI will verify

## Adding a New QUANTA-UNIVERSE Module

1. Create `your_module/lib.quanta`
2. Use `pub mod name {` not `pub module std::name {`
3. Test compilation: `quantac your_module/lib.quanta --target c -o /dev/null`
4. Common blockers:
   - `pub module std::X` → parsed as `mod std`, use `pub mod X`
   - Unicode box characters in comments → cause macro expansion errors
   - Forward references in impl blocks → define helpers before callers
   - `f64::INFINITY` → works, `i32::MAX` → works
   - `HashMap::values().find()` → deterministic (fixed, uses DefId ordering)

## Known Limitations

### Compiler
- **No module imports**: Each .quanta file must be self-contained. `use other_module::*` doesn't work.
- **No trait dispatch**: `impl Trait for Type` compiles but trait method calls may not resolve.
- **Forward references**: Methods must be defined before they're called within the same impl block.
- **&str parameters**: Functions taking `&str` get the value by copy, not pointer. Avoid.
- **Generic impls skipped**: `impl<T> Vec<T>` is not lowered to C (no monomorphization).

### C Codegen
- **Cross-module type names**: Types defined in child modules (e.g., `tonemap::Operator`) are referenced by bare name in parent structs. MSVC rejects the mismatch. 57 errors for spectrum.
- **Tuple typedefs**: `(f32, f32)` emits `Tuple_f32_f32` but the typedef may appear after first use.
- **Rectangle collision**: User type `Rectangle` collides with Windows API `wingdi.h`. Avoid this name.

### Python APPS
- **quanta-color not on PyPI**: Upload failed. Install from git.
- **calibrate-pro monolithic files**: 3 files exceed 300 lines (2,736 / 2,334 / 2,036). Refactoring planned.
- **GUI tests on Linux**: PyQt6 needs `libegl1 libxkbcommon0` on Ubuntu CI.

## 7-Gate Checklist (run before every release)

1. **Every claim has a test** — verify locally, not just CI
2. **No fabricated content** — grep for patent/production-ready/enterprise-grade
3. **Error handling is a strategy** — no bare `except Exception: pass`
4. **Incomplete is documented** — STATUS.md for anything partial
5. **Git hygiene** — no .bak, no (1).md, LICENSE present
6. **AI writes data, you write architecture** — design decisions documented
7. **GitHub pages** — descriptions, topics, CI badges, releases
