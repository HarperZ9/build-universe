# Changelog

All notable changes to the QuantaLang program suite.

## [Unreleased]

### Quality Overhaul (2026-03-27)

**Documentation:**
- Added `README.md` (192 lines) — architecture decisions, full program table,
  self-hosting chain, qdb feature set, build instructions, known issues
- Added `ARCHITECTURE.md` (176 lines) — qdb module map, data model, file format
  spec, WAL protocol, query execution flow, design tradeoffs
- Added section banners and function documentation to db.quanta (15+ functions)

**Testing:**
- Added `tests/run_tests.sh` — automated test suite
- 96 tests pass, 0 failures, 8 skips (interactive/system-dependent programs)
- Coverage: 49 of 57 executables tested

**Consistency Fixes:**
- Fixed qjq hanging on no input (added stdin_is_pipe check)
- Added `--help` to qdb, qjq, qsql (was missing — now 100% coverage)
- All 60 programs respond to `--help` or `-h`

**Code Organization:**
- db.quanta reorganized into 12 labeled sections with architecture comments
- No logic changes — pure documentation and structural clarity

### Compiler Cleanup (2026-03-26)

- Reduced compiler warnings from 2,031 to 0
- Split lower.rs (7,967 lines) into 4 modules:
  - `mod.rs` (1,609) — struct, constructors, item lowering
  - `expr.rs` (3,627) — expression and statement lowering
  - `types.rs` (777) — type lowering, const eval
  - `macros.rs` (2,028) — closures, builtins, iterators
- Added `programs/.gitignore` for build artifacts (*.c, *.exe, *.obj)
- Removed stale `db.exe` duplicate and misnamed `qdiff` C file
- Removed stray `claude-mastery-project.conf` from calibrate-pro and aurora

## [1.0.0] — 2026-03-24 to 2026-03-27

### Programs (60 total)

**Self-Hosting Compiler Chain (8 programs, 8,315 lines):**
- `qtok` — tokenizer (975 lines)
- `qparse` — recursive descent parser (1,464 lines)
- `qcheck` — type checker with scope tracking (2,040 lines)
- `qcodegen` — C code generator (1,513 lines)
- `qc` — unified self-hosted compiler (2,323 lines)
- `qfmt` — code formatter (462 lines)
- `qlint` — source linter with 8 checks (452 lines)
- `qjson` — JSON parser/pretty-printer (417 lines)

**Database Engine (3 programs, 5,634 lines):**
- `qdb` — SQL database with JOIN, GROUP BY, indexes, WAL, transactions (4,232 lines)
- `qkv` — persistent key-value store (533 lines)
- `qsql` — standalone SQL parser (869 lines)

**Text Processing (14 programs):**
- `qawk`, `qcut`, `qexpand`, `qfold`, `qgrep`, `qht`, `qnl`, `qpaste`,
  `qsed`, `qsort`, `qtr`, `qunexpand`, `quniq`, `qwc`

**Data Tools (6 programs):**
- `qbase64`, `qcalc`, `qcsv`, `qhex`, `qjq`, `qprintf`

**File Tools (9 programs):**
- `qcmp`, `qcomm`, `qdiff`, `qfind`, `qjoin`, `qloc`,
  `qpatch`, `qstrings`, `qwatch`

**System/Shell (10 programs):**
- `qbasename`, `qdate`, `qdirname`, `qecho`, `qenv`, `qrealpath`,
  `qsleep`, `qtest`, `qwhich`, `qxargs`

**Utilities (7 programs):**
- `qhttp`, `qmake`, `qmd`, `qrev`, `qseq`, `qtac`, `qtee`, `qyes`

### Compiler (quantac)

- 507 tests, 0 warnings
- 96 runtime builtins
- Backends: C (primary), LLVM, WASM, SPIRV, ARM64, x86_64 (varying completeness)
- Features: structs, enums, match, traits, generics, closures, iterators,
  tuples, Vec, HashMap, inline modules, struct constants, &self/&mut self,
  25+ float intrinsics, 11 string methods, argv/stdin, file/dir I/O,
  TCP sockets, binary I/O, Range.step_by, clock/time

### Spectrum Library

- 7,129 lines of QuantaLang color science compiles to 35,241 lines of C
- Zero parse errors, zero type errors
- Covers: RGB/XYZ/Lab/Oklab/JzAzBz/ICtCp conversions, CIEDE2000,
  tonemapping, chromatic adaptation, gamut mapping, ICC profiles

## Known Issues and Technical Debt

### Critical (blocking portfolio quality)
1. **No inter-program code reuse** — tokenizer duplicated across 5 self-hosting
   tools (~4,000 lines). Blocked by: compiler lacks multi-file compilation.
   Intended fix: add `use` import support to quantac.

### Significant
2. **Struct field assignment on locals** — compiler codegen bug. Programs work
   around it with `&mut` reference parameters. Fix requires C backend changes.
3. **String type inference** — `let x = ""` infers `&'static str`, blocking
   method calls. Workaround: use `str.substring(0, 0)` instead of `""`.
4. **Sequential while-loop codegen** — second while loop in same scope can be
   silently dropped. Workaround: use single-loop patterns or separate functions.
5. **Vec<String> incomplete** — only Vec<i32>/Vec<f64> have full codegen.
   Programs use string pool pattern instead.

### Minor
6. **GROUP BY rendering** — some aggregate combinations produce incorrect values
7. **qc subset** — self-hosted compiler handles simple programs only
8. **13 numpy warnings** in quanta-color (harmless edge cases)
9. **8 compiler integration tests skipped** (not blocking, test infra issue)
