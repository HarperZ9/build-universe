# AGENTS.md - Quanta Universe

## Scope

This file applies to the Quanta Universe repository. Root workspace instructions
still apply.

## Product Boundary

Quanta Universe is a public ecosystem map and legacy module archive for the
Build ecosystem. It contains historical `.quanta` modules, organism tooling,
calibration work, rendering/color modules, OS/kernel experiments, finance
models, and release metadata.

Current public language names are BuildLang, `buildc`, and `.bld`. Use those
names for new forward-facing docs. Historical `.quanta` paths may remain when
they describe existing files or migration work.

## Editing Rules

- Keep `STATUS.md` as the canonical maturity ledger.
- Keep `README.md`, `USAGE.md`, `CHANGELOG.md`, `tools/components.toml`, and
  `tools/package-index.toml` aligned when public status changes.
- Do not overstate whole-ecosystem buildability.
- Treat generated caches, local calibration files, live measurement artifacts,
  and private operator records as local-only.

## Verification

Run the slice that matches the change:

```powershell
python tools/verify_organism.py --quick
python tools/release_plan.py
git diff --check
```

Before committing, scan changed files for credential-shaped content.
