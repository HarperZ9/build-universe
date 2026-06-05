# photon-frametrace

A symbol table for the D3D11 immediate-context frame.

The D3D11 immediate context is a giant implicit mutable state machine. Whether a
resource is bound as a shader-resource view (read) at the same moment it is also
bound as a render target, depth-stencil, or unordered-access view (write) is an
emergent property of every Set/Clear/Unbind that ran before the draw. That is
exactly the kind of long-range mutable state an LLM cannot track in its head, and
neither can a human without tooling, which is why RenderDoc and PIX exist.

This crate is that tooling as a small, tested state machine. Feed it the event
stream (from a D3D11 vtable hook, or a RenderDoc/PIX/ETW trace), then query
ground truth instead of reasoning about it:

- what is bound to PS t27 right now (srv_at)
- the live render targets, depth-stencil, and UAVs
- which resources are simultaneously read and written at this draw (hazards)

## Status

v0 core plus tests, verified under cargo. This machine has no C compiler, so the
eventual C++ D3D11 hook layer is not built here; the Rust core exposes a surface
a thin C ABI shim can call, which is how photon/refract will drive it.

Hazard model (v0): a resource bound as an SRV (read) and also as RTV/DSV/UAV
(write) is reported. Read-only DSV and write-write (RTV plus UAV) conflicts are
known gaps; DSV is treated conservatively as a write.

## Run

    cargo test
    cargo run --example ssr_hazard

## Why it exists

An LLM has no symbol table and no heap; it approximates inference over text, so
it breaks exactly where state is mutated far from where it is read. The fix is
not to make it reason harder about state but to make it observe ground truth.
This crate is the observable substrate for the D3D11 frame.
