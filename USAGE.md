# Usage

Quanta Universe is a **mixed-language ecosystem**, not a single installable
package. There is no `pip install` / `npm install` / `cargo install` for the
whole repo. The two surfaces you actually invoke are:

1. **`quantac`** — the QuantaLang compiler (Rust), which transpiles a `.quanta`
   module or program to C, optionally compiled further to a native binary.
2. **The Python organism tooling** in `tools/` — `verify_organism.py` (ground-truth
   build/test report) and `release_plan.py` (split-repo release view).

> Authoritative per-module reality lives in [STATUS.md](STATUS.md). Where any doc
> disagrees with STATUS.md, STATUS.md wins. The commands below use only the
> invocation forms that appear verbatim in this repo (`tools/components.toml`,
> `programs/build_all.bat`, `ENGINEERING.md`, `tools/coherence/compiler_oracle.py`).

---

## Prerequisites

| Surface | Requirement |
|---------|-------------|
| `quantac` compiler | Build from the separate repo [HarperZ9/quantalang](https://github.com/HarperZ9/quantalang): `cargo build --release` → `quantalang/compiler/target/release/quantac.exe`. Not bundled here. |
| Native `.exe` from generated C | A C toolchain — MSVC `cl.exe` (Visual Studio 2022 Build Tools) or `gcc`. |
| Organism / release tooling | Python 3.11+ (`tomllib` is used, stdlib since 3.11). No third-party deps. |

The organism tooling runs with no compiler present — components that need
`quantac` or MSVC report `SKIP` honestly rather than failing.

---

## Install / build line

```sh
# Organism tooling — nothing to install, just run with Python 3.11+
python tools/verify_organism.py --quick

# Compiler (separate repo) — build once, then put quantac on PATH
git clone https://github.com/HarperZ9/quantalang
cd quantalang/compiler && cargo build --release
# binary: quantalang/compiler/target/release/quantac(.exe)
```

---

## The compilation pipeline

```
source.quanta  -->  quantac  -->  program.c  -->  cl.exe /O2 (or gcc)  -->  program.exe
                    (Rust)        (generated)      (C compiler)              (native)
```

`quantac` lowers QuantaLang to C; the generated C is then compiled with any
standard C compiler. (From `programs/README.md`.)

---

## Example 1 — Check what actually builds (no compiler required)

```sh
python tools/verify_organism.py --quick
```

Runs every component listed in `tools/components.toml`, in its directory, and
reports observed results. Exit code = number of failures (so it doubles as a CI
gate). `--quick` skips the heavy compiler build; `--json` emits a machine-readable
summary.

Expected output (illustrative — shape per the script's own formatter; the exact
PASS/SKIP set depends on whether `quantac` and MSVC are present locally):

```
Quanta organism -- verifiable components (ground truth)

  component         lang    tier  expect   result        fresh  time
  ------------------------------------------------------------------------
  frametrace-core   rust    T1    tested   PASS          fresh   3.4s
  quantalang        rust    T0    tested   SKIP:quick    -          -
  spectrum          quanta  T1    build    SKIP:no-quantac  -       -
  ...

ORGANISM: N passed, 0 failed, M skipped
```

When `quantac` is absent, the `.quanta` modules show `SKIP:no-quantac` rather
than failing — that is the intended honest behavior, not an error.

---

## Example 2 — Transpile a self-contained program to C

`programs/echo.quanta` is `qecho`, a self-contained `echo` clone — one of the
`programs/` set that `build_all.bat` builds to native `.exe` (STATUS.md Tier 1
counts 56 MSVC exes; the individually codegen-confirmed self-contained programs
named there are `color_test`, `wc`, and `base64`).

```sh
quantac programs/echo.quanta --target c -o /dev/null   # type-check + emit, discard output
quantac programs/echo.quanta                            # emit programs/echo.c
```

The first form (used by `tools/components.toml` and `ENGINEERING.md`; on Windows
the discard target is `nul`) checks that the program transpiles and exits 0 on
success. The second form (used by `programs/build_all.bat`) writes the generated
C next to the source.

Expected (illustrative): on success `quantac` exits 0 and, in the second form,
produces `programs/echo.c`. A type/codegen error prints a diagnostic and a
non-zero exit.

---

## Example 3 — Transpile and build a program to a native binary (Windows + MSVC)

```bat
cd programs
build_all.bat
```

`build_all.bat` runs `quantac` over every `*.quanta` in `programs/` and then
compiles each generated `.c` with MSVC `cl /O2`, emitting `q<name>.exe` (e.g.
`qecho.exe`, `qwc.exe`). It expects `quantac.exe` at
`..\quantalang\compiler\target\release\quantac.exe` and VS 2022 Build Tools.

To build a single generated C file by hand:

```bat
quantac programs\echo.quanta            REM -> programs\echo.c
programs\build.bat programs\echo.c qecho.exe
```

Expected output (illustrative — per the scripts' own echoes):

```
=== Building all QuantaLang programs ===
  Compiling echo.quanta...
  OK  qecho.exe
  ...
=== Results ===
  PASS: 56
  FAIL: ...
  SKIP: ...
```

> Not every `.quanta` module builds standalone. `tools/components.toml` documents
> which modules transpile on their own; modules like `photon`, `quantum-finance`,
> `axiom`, and `forge` are intentionally **not** registered as buildable (see the
> header comment in that file and STATUS.md).

---

## Example 4 — Verify a single module transpiles

Each domain module exposes `lib.quanta`. The organism check for a module is
exactly its `verify` command from `tools/components.toml`, e.g. for `spectrum`:

```sh
cd spectrum
quantac lib.quanta --target c -o /dev/null    # nul on Windows
```

Exit 0 means the module transpiles to C standalone. This is a `build`-level
check (transpile/exit-0), not proof the emitted C compiles end-to-end — see the
codegen-oracle note in STATUS.md for that deeper, separate witness.

---

## Release tooling (maintainers)

`tools/release_plan.py` turns `tools/package-index.toml` into a release view:

```sh
python tools/release_plan.py --only-publish --json
python tools/release_plan.py --module axiom --json
python tools/release_plan.py --only-publish --write-markdown
```

See [tools/README.md](tools/README.md) for the full tooling overview.
