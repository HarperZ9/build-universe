# qdb has moved

qdb, the SQL engine written in BuildLang (BuildDB), now lives in HarperZ9/buildlang as `bdb/`:
https://github.com/HarperZ9/buildlang/tree/main/bdb

It moved on 2026-11-09 with its history, so `git log --follow bdb/db.bld` in buildlang shows every
commit it had here. There it is built by the compiler of each buildlang checkout and its tests run
in that repository's CI (`.github/workflows/bdb.yml`).

The last commit of this repository that holds the source is 4a296c1: `programs/db.bld`, its
architecture notes (`programs/ARCHITECTURE.md`), its tests (`programs/tests/test_qdb*.{sh,py}`),
its benchmarks (`programs/benchmarks/bench_qdb*`) and its workflow (`.github/workflows/qdb.yml`).
Its changelog entries before the move stay in `programs/CHANGELOG.md`.
