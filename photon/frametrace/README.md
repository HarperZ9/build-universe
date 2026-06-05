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

v1 core plus tests (13 tests, all passing under cargo). The Rust core exposes a
surface a thin C ABI shim can call; the C++ D3D11 vtable hook that feeds it is
the next step (MSVC is now available locally).

Hazard model (v1): a resource bound as an SRV (read) and also as RTV/DSV/UAV
(write) is a ReadWrite hazard; a resource bound through two or more distinct
write views with no reader (e.g. RTV plus UAV) is a WriteWrite hazard. A
read-only DSV is correctly treated as neither read nor write. Each draw/dispatch
snapshots its hazards into a per-frame log (hazard_log / hazards_at).

## Run

    cargo test
    cargo run --example ssr_hazard

## Why it exists

An LLM has no symbol table and no heap; it approximates inference over text, so
it breaks exactly where state is mutated far from where it is read. The fix is
not to make it reason harder about state but to make it observe ground truth.
This crate is the observable substrate for the D3D11 frame.

## C ABI (for the D3D11 hook)

include/frametrace.h declares an extern "C" surface (ft_new, ft_register_view,
ft_set_shader_resources, ft_set_render_targets, ft_draw, ft_hazard_count, ...)
so a C or C++ D3D11 vtable hook can drive the Rust core across FFI. View and
resource ids are the ID3D11 pointers cast to uint64; id 0 means null/unbound.

Build the staticlib, then the C demo (Windows + MSVC):

    cargo build
    cmd /c c_demo\build.bat
    c_demo\demo.exe

c_demo/demo.c reproduces the SSR hazard through the C ABI and is verified to
compile (MSVC) and run, linking target/debug/photon_frametrace.lib. The required
native libs (kernel32 ntdll userenv ws2_32 dbghelp + msvcrt) come from
cargo rustc --lib -- --print native-static-libs.

Next: an ID3D11DeviceContext vtable hook (MinHook) that emits these events from a
live process. That step needs a running D3D11 app to exercise, so it is built
here but its runtime is validated against a target, not in CI.

## Trace ingestion (offline capture analysis)

The trace feature (on by default) ingests a normalized JSON trace -- what a
RenderDoc / PIX / ETW adapter emits -- and replays it into a FrameState:

    cargo run --example replay_trace -- tests/data/ssr_trace.json

A trace is a JSON array of op-tagged events: register_view, set_shader_resources,
set_render_targets, set_uav, clear_rtv, clear_dsv, draw, dispatch. Ids are u64
(0 means null/unbound). FrameState::replay_json applies the whole trace; then
query hazard_log / hazards_at for the per-draw verdict. For a dependency-free
core, build with --no-default-features (drops serde and trace ingestion).
