# Demo -- hello_quanta

> **Best-effort demo -- not runtime-verified by author.**
> Commands use only the invocation forms that appear verbatim elsewhere in this
> repo (`programs/build_all.bat`, `programs/build.bat`, `tools/components.toml`,
> `ENGINEERING.md`). The expected output below is illustrative; it was not
> executed while writing this file. The `quantac` compiler lives in the separate
> [HarperZ9/quantalang](https://github.com/HarperZ9/quantalang) repo and is not
> bundled here.

[`hello_quanta.quanta`](hello_quanta.quanta) is a minimal, self-contained
QuantaLang program (no arguments, no imports) built from the same constructs as
`programs/test_hello.quanta`: free functions, recursion, `let mut`, a `while`
loop, i64 arithmetic, and `println!` formatting.

## 1. Transpile to C (needs `quantac` on PATH)

```sh
quantac examples/demo/hello_quanta.quanta --target c -o /dev/null   # type-check only (nul on Windows)
quantac examples/demo/hello_quanta.quanta                            # emit hello_quanta.c
```

The first form exits 0 if the program transpiles cleanly. The second writes
`hello_quanta.c` next to the source.

## 2. Build a native binary (Windows + MSVC)

```bat
quantac examples\demo\hello_quanta.quanta
programs\build.bat examples\demo\hello_quanta.c demo.exe
demo.exe
```

`programs\build.bat input.c [output.exe]` invokes `cl /O2` via the VS 2022 Build
Tools (see the script for the exact `vcvars64.bat` path).

With `gcc` instead of MSVC:

```sh
quantac examples/demo/hello_quanta.quanta   # -> examples/demo/hello_quanta.c
gcc -O2 examples/demo/hello_quanta.c -o demo
./demo
```

## Expected output (illustrative)

```
Hello from QuantaLang
3628800
5050
```

## Notes

- `factorial(10)` = 3628800 and `sum_to(100)` = 5050 are arithmetic facts,
  independent of the toolchain.
- See [../../USAGE.md](../../USAGE.md) for the full command surface and
  [../../STATUS.md](../../STATUS.md) for which modules/programs are verified.
