# Contributing

Quanta Universe is a mixed legacy ecosystem. Contributions should make the repo
more inspectable, more honest, and easier to migrate into the BuildLang /
Project Telos toolchain.

## Local checks

```bash
python tools/verify_organism.py --quick
python tools/release_plan.py
git diff --check
```

## Boundaries

- Keep `STATUS.md` canonical for maturity claims.
- Keep public names clear: BuildLang is the current language, `buildc` is the
  current compiler binary, and `.bld` is the current source extension.
- When touching historical `.quanta` modules, document whether the change is a
  migration, bridge, or preservation edit.
- Do not claim the whole ecosystem builds unless the organism verifier proves it.
- Do not commit generated caches, secrets, local calibration files, or private
  operator state.
