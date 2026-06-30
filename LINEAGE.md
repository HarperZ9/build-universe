# Quanta Family -- Lineage and Interlacing (Canonical)

Last verified: 2026-06-05. Maps the related workspaces, how the mixed languages
hand off to each other, and the foundational-to-derivative direction. Single
source of truth for the family tree.

## Roster

| Workspace | Path | Language(s) | Remote | Role |
|---|---|---|---|---|
| QUANTA-UNIVERSE | C:/Users/Zain/QUANTA-UNIVERSE | QuantaLang (.quanta), Rust | HarperZ9/quanta-universe | Foundational ecosystem + spec |
| quantalang | nested, git-ignored in UNIVERSE | Rust | HarperZ9/quantalang | The compiler (separate repo) |
| APPS | C:/Users/Zain/APPS | Python | HarperZ9/APPS (+7 submodules) | Derivative product layer |
| quanta-color | APPS submodule | Python | HarperZ9/quanta-color | Color product (published) |
| calibrate-pro | APPS submodule | Python + vendored C/C++ | HarperZ9/calibrate-pro | Display-calibration product (shipped exe) |
| quanta-finance / -oracle / -engine / -ui / -ecosystem | APPS submodules | Python | HarperZ9 | Finance / forecast / engine / UI / meta |
| aurora | referenced only | Lua (intended) | -- | No source present locally |

## Lineage direction (evidence-backed)

QUANTA-UNIVERSE (foundational) -> APPS (derivative productization).

- UNIVERSE commit history predates the APPS initial commit.
- The dwm_lut native wrapper exists in UNIVERSE first; APPS carries a
  near-identical derived copy (same algorithms, ~1,190 vs ~1,185 LOC).
- Every APPS subproject re-implements concepts expressed first in a UNIVERSE
  .quanta module. No UNIVERSE module depends on APPS.
- All .quanta modules depend on the quantalang compiler; APPS has zero .quanta
  dependency (hand-written Python, not generated from .quanta).

Correction to a common misstatement: APPS does NOT import the .quanta modules
(Python cannot). APPS is a parallel, hand-written re-implementation in Python of
the same algorithms -- a productization, not a binding.

## How the languages interlace (handoff chain)

    Rust (quantalang compiler)
       |  compiles
       v
    QuantaLang (.quanta + programs/) --transpile--> C --> MSVC --> native .exe
       |                                                  (56 verified programs)
       |  same algorithms re-expressed by hand
       v
    Python (APPS products) --ctypes--> vendored C/C++ DLLs
       |                               (dwm_lut: MinHook + HDE) --> Windows DWM
       v
    Lua (aurora): intended scripting layer, not yet present

Module correspondence (UNIVERSE .quanta  to  APPS Python):

| UNIVERSE module | APPS product |
|---|---|
| calibrate (+ vendored dwm_lut) | calibrate-pro |
| spectrum, chromatic | quanta-color |
| delta, entropy, field-tensor | quanta-finance |
| oracle | quanta-oracle |
| neutrino | quanta-engine |
| nova, lumina | quanta-ui |

## Verified APPS test counts (vs prior ENGINEERING.md claims)

| Product | Claimed | Measured | Note |
|---|---|---|---|
| calibrate-pro | 297 | 228 | over-claimed |
| quanta-color | 457 | 281 | over-claimed |
| quanta-finance | 142 | 142 | match |
| quanta-oracle | 187 | 187 | match |
| quanta-engine | 96 | 173 | under-claimed |
| quanta-ui | 17 | 17 | match |
| aurora | Lua | 0 | no source present |

## Combining the family -- recommendation

Do NOT physically merge git histories or collapse the repos yet. The pieces are
intentionally separated by language and release cadence (UNIVERSE = spec,
quantalang = compiler, APPS = shippable Python with its own submodules). A blind
merge would shred the submodule structure and the .quanta/Python boundary.

Recommended combination mechanism, safest first:

1. Lineage manifest (this file) -- one authoritative map. Done.
2. Shared golden-vector tests -- extract the verifiable kernels (color, options,
   SARIMA, SHA-256) into language-neutral test vectors that BOTH the .quanta and
   the Python implementations must satisfy. This is the real combine: it binds
   the two language families by behavior, not by moving files.
3. Optional umbrella repo -- add UNIVERSE and APPS as submodules of a thin parent
   (or a git subtree) only if a single clone is required. Reversible.

Physical history merge is out of scope until explicitly requested.
