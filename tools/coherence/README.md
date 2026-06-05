# Coherence-membrane organs (cross-domain)

The Build/Compiler organ of the coherence membrane (docs/COHERENCE-MEMBRANE.md),
the same gate the GPU organ (photon/frametrace) exposes via ft_adjudicate, here
for build state.

## freshness.py -- build-state coherence adjudicator

The claim "the verified artifact reflects current source" is adjudicated against
a content-hash witness:

- CONFIRMED   -- current source hash == the hash recorded at the last good verify;
- CONTRADICTED -- they differ: the artifact is STALE (your edit is not reflected);
- UNRESOLVABLE -- no recorded build: cannot adjudicate freshness.

The witness hashes source CONTENT, so it is independent of mtime-based build-tool
fingerprints -- it catches the exact 2026-06-05 failure where cargo replayed a
stale binary while reporting "up to date" because the IO layer preserved mtimes.
Method diversity (the content hash AND the build tool) is the point: when they
disagree, do not trust the green.

verify_organism.py maintains these witnesses automatically (records on PASS) and
shows a freshness column, so "did my edit get verified" is a witnessed verdict.
Witness records live in witness/ (git-ignored, per-machine build state).
