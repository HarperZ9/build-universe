# QuantaLang Programs

60 CLI tools compiled from QuantaLang to native binaries via C codegen.

Total: ~30,900 lines of QuantaLang source across 60 programs.

## Architecture

### Compilation Pipeline

Programs are compiled in two steps:

```
source.quanta  -->  quantac  -->  program.c  -->  cl.exe /O2  -->  program.exe
                    (Rust)        (generated)      (MSVC)          (native)
```

`quantac` is the Rust-based compiler that lowers QuantaLang to C. The generated
C is then compiled with any standard C compiler (MSVC `cl.exe` or `gcc`).

### Why Flat Structs

QuantaLang's C backend has a known limitation: struct field assignment on local
variables doesn't emit correct C in all contexts. As a workaround, all programs
use a "flat struct" pattern where state lives in a single struct passed via `&mut`
references to helper functions. This trades readability for reliability.

Example from `qdb` -- all parser state, storage, indexes, and WAL live in one
`DB` struct (~50 fields) rather than separate `Parser`, `Storage`, `Index` types.

### Why String Pools

QuantaLang's `Vec<String>` support is incomplete (only `Vec<i32>` and `Vec<f64>`
have full codegen support). Programs that need arrays of strings use a "string
pool" pattern: a single `String` buffer with parallel `Vec<i32>` arrays tracking
start positions and lengths. This is similar to arena allocation.

```
sp_buf:    "alicebobcharlie"
sp_starts: [0, 5, 8]
sp_lens:   [5, 3, 7]
```

A call to `sp_get(state, id)` extracts `sp_buf.substring(sp_starts[id], sp_lens[id])`.

### Why Embedded Tokenizers

The compiler doesn't yet support multi-file compilation or `use` imports between
`.quanta` files. Programs that need tokenization (parse, check, codegen, qc)
embed their own tokenizer. This is acknowledged technical debt -- a shared stdlib
is the intended fix.

## Programs

| Program | Lines | Category | Description |
|---------|------:|----------|-------------|
| qawk | 896 | text | AWK-lite text processing tool |
| qbase64 | 238 | encoding | Base64 encode/decode |
| qbasename | 147 | path | Strip directory and suffix from filenames |
| qcalc | 752 | math | Mathematical expression calculator |
| qcheck | 2,040 | compiler | QuantaLang type checker (self-hosting) |
| qcmp | 200 | file | Byte-by-byte file comparison |
| qcodegen | 1,513 | compiler | QuantaLang-to-C code generator (self-hosting) |
| qcomm | 319 | text | Compare two sorted files line by line |
| qcsv | 595 | data | CSV-to-JSON converter |
| qcut | 476 | text | Field/character extraction (GNU cut) |
| qdate | 156 | system | Display current date and time |
| qdb | 4,232 | database | SQL database engine with REPL |
| qdiff | 718 | file | Unified diff (LCS-based) |
| qdirname | 146 | path | Extract directory component from pathname |
| qecho | 131 | system | Print arguments to stdout |
| qenv | 109 | system | Environment variable viewer |
| qexpand | 152 | text | Convert tabs to spaces |
| qfind | 298 | file | File search utility |
| qfmt | 462 | dev | Code formatter for QuantaLang source |
| qfold | 190 | text | Wrap long lines to specified width |
| qgrep | 224 | text | Pattern matching (simplified grep) |
| qhex | 262 | encoding | Hex dump utility |
| qht | 205 | text | Combined head+tail utility |
| qhttp | 239 | net | HTTP client (httpie-lite) |
| qjoin | 560 | text | Join lines of two files on a common field |
| qjq | 1,014 | data | jq-lite JSON query engine |
| qjson | 417 | data | JSON parser/pretty-printer |
| qkv | 533 | database | Persistent key-value database CLI |
| qlint | 452 | dev | QuantaLang source linter |
| qloc | 489 | dev | Lines-of-code counter (tokei/scc) |
| qmake | 747 | dev | Make-lite build tool |
| qmd | 843 | data | Markdown-to-HTML converter |
| qnl | 208 | text | Number lines (GNU nl) |
| qparse | 1,464 | compiler | QuantaLang parser (self-hosting) |
| qpaste | 371 | text | Merge lines of files (GNU paste) |
| qpatch | 198 | file | Apply unified diff patches |
| qprintf | 224 | text | Format string output |
| qc | 2,323 | compiler | Self-hosted QuantaLang compiler (all stages) |
| qrealpath | 142 | path | Normalize path, resolve . and .. |
| qrev | 126 | text | Reverse characters in each line |
| qsed | 698 | text | Stream editor (sed) |
| qseq | 310 | math | Print a sequence of numbers |
| qsleep | 105 | system | Delay for a specified time |
| qsort | 301 | text | Sort lines (GNU sort) |
| qsql | 869 | database | SQL parser (standalone) |
| qstrings | 188 | file | Extract printable ASCII strings from files |
| qsum | 167 | file | BSD-style checksum |
| qtac | 189 | text | Print lines in reverse order |
| qtee | 87 | system | Split output to file and stdout |
| qtest | 161 | system | Evaluate conditional expressions (test/[) |
| qtok | 975 | compiler | QuantaLang tokenizer (self-hosting) |
| qtr | 291 | text | Character translate/delete/squeeze |
| qunexpand | 178 | text | Convert leading spaces to tabs |
| quniq | 259 | text | Remove consecutive duplicate lines |
| qwatch | 367 | file | File snapshot and change detection |
| qwc | 186 | text | Word/line/byte counter (GNU wc) |
| qwhich | 141 | system | Locate a command on PATH |
| qxargs | 533 | system | Command builder from stdin |
| qyes | 81 | system | Repeatedly output a line |

## Self-Hosting Compiler Chain

QuantaLang can compile a subset of itself to C:

```
source.quanta --> qtok (tokenize) --> qparse (AST) --> qcheck (typecheck) --> qcodegen (emit C)
```

The `qc` program (2,323 lines) combines all four stages into a single binary.
The `qcodegen` component (1,415 lines) has been proven end-to-end:

**Programs compiled by the self-hosted compiler to working binaries:**
- `yes.quanta` (81 lines) → 156 lines of C → native exe → correct output
- `echo.quanta` (131 lines) → 195 lines of C → native exe → correct output
- `test_hello.quanta` → factorial(10) = 3628800, sum(1..100) = 5050

The generated C includes an embedded runtime with string helpers (starts_with,
substring, strlen, strcmp, strcat), argument handling (argc/argv translation),
and cross-platform stdin detection.

**What qcodegen handles:** functions, let/let mut, if/else/else-if, while loops,
return, break, arithmetic, comparisons, **struct definitions, struct literals,
field access, field assignment**, string operations (==, +, .len(),
.starts_with(), .substring(), .parse_int(), .char_at(), .contains()),
args_count/args_get, process_exit, file I/O builtins, println! with type-aware
format specifiers (%s for strings, %lld for ints).

**Self-hosted compiler coverage: 62/62 programs generate C output.**

All 62 programs in the suite produce valid C from `qcodegen`. End-to-end
verified (C compiles and runs correctly): test_hello, yes, echo, basename,
dirname, seq. The remaining programs generate C but may have runtime-level
type issues that require the Rust-based `quantac` for correct binaries.

**All major codegen features implemented.** The self-hosted compiler handles:
functions, let, if (ternary + blocks), while, return, structs, impl methods,
Vec, include!() preprocessing, closures/lambdas (function pointers + iterator
desugaring for map/filter/fold), 30+ builtin/method mappings, and self-compilation.

The only remaining gaps are advanced patterns: nested closures capturing
outer variables, generic type parameters, and pattern matching/enums.

The self-hosted compiler now handles: functions, let, if/else, while, return,
structs (typedef/literal/field access/assignment), impl methods (Type_method
mangling, self->field, &obj dispatch), Vec (vec!/push/get/len), include!()
preprocessing, string operations (15 methods), builtins (args, file I/O,
time, clock, getenv), println! with type-aware format specifiers.

**Self-compilation:** qcodegen processes its own 2,081-line source → 3,352 lines
of C, proving the self-hosted compiler can handle substantial QuantaLang code.

## Database Engine (qdb)

The most complex program at 4,232 lines. A SQL database engine with:

**SQL support:**
- CREATE TABLE, INSERT, SELECT, UPDATE, DELETE, DROP TABLE
- ALTER TABLE ADD COLUMN
- WHERE with AND/OR, LIKE, IN, IS NULL/IS NOT NULL, BETWEEN
- Scalar subqueries in WHERE clauses
- JOIN (nested-loop) with table aliases
- GROUP BY with aggregates: COUNT(*), SUM, AVG, MIN, MAX
- HAVING, DISTINCT, ORDER BY, LIMIT
- Column aliases (SELECT col AS alias)

**Storage and reliability:**
- Line-based file persistence format
- CREATE INDEX with sorted-array indexes and binary search lookup
- Write-Ahead Log (WAL) for crash recovery
- BEGIN/COMMIT/ROLLBACK transaction support
- Interactive REPL with dot-commands (.tables, .schema, .wal)

**Architecture:** All state (parser, storage, indexes, WAL, REPL) lives in a
single `DB` struct due to the flat-struct constraint. The SQL parser is embedded
(not shared with qsql) because there is no multi-file import support.

**Known limitations:**
- GROUP BY has rendering issues with some aggregate combinations
- No query optimizer; all scans are sequential unless an index matches exactly
- File format is not binary-compatible with SQLite

## Key-Value Store (qkv)

A persistent key-value database (533 lines) with GET, SET, DELETE, LIST, and
DUMP operations. Uses append-only file storage with compaction.

## Building

```bash
# Compile a single program
quantac programs/wc.quanta
cl.exe /O2 /Fe:programs/qwc.exe programs/wc.c

# Or with GCC
gcc -O2 -o programs/qwc.exe programs/wc.c
```

The `build.bat` script in this directory compiles all programs.

## Testing

```bash
bash programs/tests/run_tests.sh
```

96 automated tests cover 49 of 57 executables. 8 programs are skipped
(interactive, network-dependent, or long-running). Zero failures.

## Project Status

| Metric | Value |
|--------|-------|
| Programs | 60 |
| Total QuantaLang LOC | ~30,900 |
| Automated tests | 96 pass, 0 fail |
| Compiler tests | 507 pass, 0 warnings |
| Self-hosting tools | 7 (fmt, lint, tok, parse, check, codegen, qc) |
| Spectrum compilation | 7,129 lines compiles to 35,241 lines of C |

## Roadblocks and Watch List

### ~~Blocking: Multi-file Compilation~~ RESOLVED (2026-03-27)
Added `include!("path")` preprocessor directive. Programs can now share code:
```quanta
include!("stdlib/chars.quanta");
// is_digit(), is_alpha(), etc. are now available
```
Double-inclusion guard prevents duplicates. Stdlib modules:

```
stdlib/chars.quanta        — is_digit, is_alpha, is_alnum, is_whitespace,
                             is_hex_digit, is_bin_digit, is_oct_digit
stdlib/tokenizer.quanta    — Tok struct, 48 TK_* constants, tokenize(),
                             read_string(), read_number(), peek/advance (430 lines)
stdlib/string_utils.quanta — trim_left, starts_with_alpha
stdlib/lines.quanta        — LineReader (split string into lines)
stdlib/string_pool.quanta  — StringPool (string array via Vec<i32>)
```

Self-hosting tools (tok, parse, check, codegen, qc) migrated to shared
stdlib — **2,358 total lines of duplication eliminated** (240 from char
functions + 2,118 from tokenizer).

### ~~Blocking: Struct Field Assignment on Locals~~ FIXED (2026-03-27)
Added `MirStmtKind::FieldAssign` across IR, lowerer, and all backends.
`p.x = 10;` now emits correctly in any function scope. Programs can be
refactored to remove `&mut` setter workarounds.

### Monitoring: String Type Inference
`let x = ""` infers `&'static str` instead of `String`, which blocks method
calls like `.char_at()`. Programs use `str.substring(0, 0)` as a workaround.
This causes subtle bugs when mixing string literals with string operations.

### ~~Monitoring: Sequential While Loops~~ FIXED (earlier session)
The var_map save/restore fix resolved this. Three sequential while loops now
generate correct C with proper basic block chains. Verified.

### Monitoring: Vec<String> Codegen
Only `Vec<i32>` and `Vec<f64>` have complete codegen. Programs that need string
arrays use the string pool pattern (string buffer + parallel index arrays).

## Known Issues

- GROUP BY has rendering issues with some aggregate combinations
- Self-hosted compiler (qc) handles a subset of the language only
- 3 programs (qc, json, test_hello) have `.c` output but no linked binary
  (requires manual MSVC invocation)

## Building

### With the Rust compiler (quantac):
```bash
quantac program.quanta        # generates program.c
cl /O2 /Fe:qprogram.exe program.c   # MSVC
gcc -O2 -o qprogram program.c       # GCC
```

### Batch build (all programs):
From cmd.exe: `cd programs && build_all.bat`

### Self-hosted build (via qcodegen):
From cmd.exe: `cd programs && build_all_self.bat`

## Documentation

- **[Website](https://harperz9.github.io/quanta-universe)** — overview, examples, architecture
- `CHANGELOG.md` — detailed change history
- `ARCHITECTURE.md` — qdb database engine architecture
- `tests/run_tests.sh` — automated test suite
- `benchmarks/` — qdb performance benchmarks
