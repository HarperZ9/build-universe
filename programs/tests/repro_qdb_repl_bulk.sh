#!/usr/bin/env bash
# KNOWN OPEN: qdb (BDB) crashes partway through a long REPL session.
#
# Opt-in reproduction, not part of test_qdb.sh: it takes about three minutes.
# Build qdb.exe first, then: bash programs/tests/repro_qdb_repl_bulk.sh [N]
#
# Observed 2026-10-29 with buildc 1.4.0 + gcc on Windows, piping 5,000 single-row
# INSERT statements into the REPL:
#   - this branch (577abe4): segmentation fault after 2,549 rows were stored,
#     and after 2,507 of 3,000 in a second run with this script;
#   - main's db.bld (effect-annotated scratch build): the REPL stopped after
#     2,464 rows with no message (the crash was not visible through the pipe).
# Rows stored before the crash were intact, and 4,000 rows loaded through 40
# single-command multi-row INSERTs without error, so table size alone does not
# trigger it. Something grows per REPL statement. Cause not yet found.
#
# Exit 0 when all N rows land (fixed), 1 when the REPL dies early (still open).
set -o pipefail
cd "$(dirname "$0")"
QDB="$(cd .. && pwd)/qdb.exe"
N="${1:-3000}"
DB=/tmp/qdb_repl_bulk.db
if [ ! -f "$QDB" ]; then echo "SKIP: $QDB not found"; exit 0; fi
rm -f "$DB" "$DB".wal
{
  echo "CREATE TABLE art (path TEXT, kind TEXT, size INTEGER);"
  i=0
  while [ "$i" -lt "$N" ]; do
    echo "INSERT INTO art VALUES ('p$i.md','k$((i % 7))',$i);"
    i=$((i + 1))
  done
  echo ".quit"
} | "$QDB" "$DB" >/dev/null 2>&1
rc=$?
got="$("$QDB" "$DB" "SELECT COUNT(*) FROM art" | tr -d '\r' | tail -n +3 | tr -d ' ')"
rm -f "$DB" "$DB".wal
echo "repl bulk load: rc=$rc rows=$got want=$N"
if [ "$got" = "$N" ] && [ "$rc" -eq 0 ]; then echo "FIXED"; exit 0; fi
echo "OPEN: REPL died after $got of $N statements"
exit 1
