# QUANTA-UNIVERSE — Module Maturity Ledger (Canonical)

Last verified: 2026-06-05. This file is the single source of truth for module
reality. Where README, ENGINEERING, CHANGELOG, UNIVERSE.toml, or CATALOG.json
disagree, this file wins. Scores are engineering concreteness (0-10) from direct
source audit: real, correct, extractable logic vs scaffolding/showcase.

## Canonical facts

- Version: 1.0.0  |  License: MIT (see LICENSE)
- Compiler: 612 tests pass / 0 fail on cargo test (755 #[test] annotations incl. ignored/multi-bin); only the C backend is end-to-end.
  HLSL/GLSL emit text; x86-64/ARM64/WASM/LLVM/SPIR-V emit output but have no
  linker/assembler integration (no runnable artifacts yet).
- Each .quanta module transpiles to C individually. Whole-ecosystem cross-module
  compilation and compiler self-hosting are not yet achieved.
- ~6 GB of local disk is Cargo target/ build cache (already git-ignored; the
  compiler dir quantalang/ is itself git-ignored, a separate repo).

## Tier 1 — Load-bearing, real, extractable (the engine)

| Component | Score | What is real |
|---|---|---|
| quantalang compiler (C backend) | 6.5 | Full front-to-C pipeline; monomorphization, traits/vtables, one-shot effects; 612 pass / 0 fail |
| programs/ (56 MSVC exes) | 9.0 | qdb (SQL), qparse, qsed, grep, base64, calc, color_test 12/12 |
| spectrum (color science) | 9.0 | sRGB/XYZ matrices, PQ/HLG EOTFs, 13 tonemappers, verified OKLab constants |
| chromatic (perceptual color) | 8.0 | LAB/RGB with matrix inversion, gamut mapping via binary search |
| delta (options pricing) | 8.0 | Black-Scholes exact, full Greeks, Newton-Raphson + Brent IV |
| foundation math / crypto | 8.0 / 7.0 | SHA-256 FIPS 180-4 correct; trig/pow/log via intrinsics |

## Tier 2 — Real kernel inside scaffolding (extract the core)

| Component | Score | Real core | Scaffolding |
|---|---|---|---|
| quantaos kernel | 6.5 | Memory, scheduler, ext2/4, IPC, drivers, TCP/IP stack | AI syscalls return -1; self-healing is Z-score, not ML |
| entropy | 7.0 | LSTM forward pass; GBDT variance splits | No backprop |
| oracle | 7.0 | SARIMA differencing + AR/MA fitting | Forecast integration thin |
| field-tensor | 6.0 | Cholesky, power-iteration eigenvalues, indicators | 4D market application sparse |
| quantum-finance | 6.0 | OHLCV, TWAP/VWAP, risk calcs | Broker APIs return empty data |
| photon | 6.0 | ~15% real: meshlet culling, attenuation, ray differentials | ~85% boilerplate; hooks return false |
| prism | 7.0 | Correct HLSL tonemapper math | No GPU compile/injection |
| axiom | 5.0 | Forward-mode dual-number autodiff correct | Mutation ops stubbed; MAML/CMA-ES sketched |
| foundation collections | 6.0 | Vec correct | HashMap/BTreeMap sparse; regex executor missing |

## Tier 3 — Sketch / showcase (design intent, not engineering)

These remain in-tree (interlaced into build/interconnect wiring) but are not to
be represented as implemented engineering.

| Component | Score | Reality |
|---|---|---|
| lumina | 5.0 | Preset configs; claimed FFT bloom has no FFT code |
| refract | 5.0 | ENB metadata; no D3D11 hook installation |
| forge | 5.0 | Logger real; linter/debugger/profiler are shells |
| neutrino | 4.0 | Neural rendering is type enums; zero tensor ops |
| nexus | 4.0 | Mod-framework skeleton; no loader/solver |
| wavelength | 4.0 | Media containers; no DSP/codec |

## Infrastructure dirs (not domain modules)

cli, config, runtime, repl, lsp, fmt, debug, test, profiler, bench, benchmarks,
pkg, docs, universe, examples are scaffolding/glue for the ecosystem manifest,
of mixed completeness. Treat as supporting tooling, not headline modules.

## Compiler capabilities verified 2026-06-05 (Phase 1)

A local C toolchain is now available (MSVC -- Visual Studio 18 Community / 2022
BuildTools via vcvars64.bat), so generated C is now NATIVELY compiled and run,
not just emitted. This immediately corrected several emission-only conclusions.

- Works (was documented as a stale limitation): &self / &mut self receivers; &str
  params; trait default methods calling self.method(); occurs-check on &Struct
  returning the same struct literal; Self{...}, Self::method().
- FIXED + native-verified: the non-generic constructor pattern
  fn new() -> Self { ... } with let x = T::new() called from a free function.
  Emission alone hid a real bug: the call result was typed as the literal Self
  (invalid C; method dispatch fell back to a bare name). Root cause was an
  impl-collect forward-declaration that did not resolve Self -> concrete type.
  Fixed in quantalang 8d83d74 + 2e9296f. e2e regressions tests/programs/132
  (prints 42) and 133 (prints 5) now compile with MSVC cl and run. Rust suite
  612 pass / 0 fail.
- KNOWN OPEN (native LINK failure): GENERIC impl methods returning Self, e.g.
  Wrap<T>::new() -> Self -- the monomorphized constructor/method are not emitted
  (unresolved external symbol). This is the next real blocker (generic
  monomorphization). Correction: an earlier note claimed generic methods
  returning Self "compile"; that was emission-only and is FALSE at native link.
- Heap types (String) in enum variants and generic enum methods: type-check +
  emit only; not yet individually native-checked.

Caveats: (1) The 231K-line self-hosted compiler in quantalang/src/ still does not
compile as a whole -- per-feature success does not imply whole-program self-hosting.
(2) Dev-env: cargo incremental builds are unreliable here (source mtimes are
preserved by the IO layer); use `cargo clean -p quantalang` before each rebuild.
(3) Native build helper: C:/Users/Zain/.cqtest/build.bat (vcvars -> cl -> run).

## Verifiable tier (machine-checked)

The load-bearing components are now machine-verifiable, not merely asserted in
this file. Run:

    python tools/verify_organism.py     (registry: tools/components.toml)

It builds and tests every real component across languages and reports ground
truth; the exit code is the failure count, so it is also a CI gate. Last run:
20 passed / 0 failed -- frametrace (core, C ABI, hook, adapter), quantalang, and 16 .quanta domain modules (verified by quantac transpile). CI runs it on windows-latest (.github/workflows/organism.yml).
New real modules join the organism by registering in components.toml.

## Compiler organ deep-codegen finding (2026-06-05)

The Compiler organ (tools/coherence/compiler_oracle.py) adjudicates the deeper
claim "the emitted C compiles", beyond the shallow "quantac exits 0". Ground
truth below, re-scanned after two codegen fixes landed.

Fixed (quantalang branch fix/codegen-module-prefix, commit 392ee5d):
- Module forward-reference prefixing. A struct or enum field whose type was
  declared later in the same inline module kept its bare name, so the emitted C
  named an undefined identifier (a bare ColorPrimaries field where only
  hdr_ColorPrimaries is defined). collect_inline_mod now forward-declares every
  module type (bare to module-prefixed) before lowering any item body. Verified:
  the spectrum field is now hdr_ColorPrimaries primaries.
- Primitive tuple field typedefs. A tuple field carried as MirType::Tuple (e.g.
  an xy chromaticity, Tuple_f32_f32) was referenced but never typedef-ed. Now
  emitted, gated to all-primitive element tuples. Tuples of named structs still
  need topological ordering (the typedef must follow the member struct full
  definition) and are left as a known gap.
- Regression: 337 codegen tests pass, 0 failed (incl. C-backend snapshots).

Membrane-wide scan (compiler_oracle.py adjudicate-all, 16 .quanta modules): no
module compiles standalone yet. The dominant blocker is single-module isolation:
the oracle compiles each module in isolation (its lib.quanta alone), so
cross-module and stdlib types are absent. Witnessed identifiers: Box (oracle,
entropy), BTreeMap (delta), Vec2 (nova), Mat4 (lumina), XYZ (calibrate), Version
(nexus), SnapshotId (entangle), ID3D11RenderTargetView (refract). This is the
documented cross-module-resolution caveat, not a codegen defect. Other real
blockers surfaced: spectrum declares pub mod harmony twice (enum redefinition, a
source defect a stricter frontend would reject); field-tensor redefines
Tuple_usize_usize; foundation applies == to QuantaString; prism, neutrino, and
wavelength fail at transpile with mutability mismatches. Self-contained programs
color_test, wc, base64 remain CONFIRMED.

Conclusion (truth-over-approval): "transpiles cleanly" (the 16 green modules in
the organism check) does NOT imply "compiles standalone". The organism gate keeps
transpile as the module bar (a real if shallow check); the compiler organ is the
separate, deeper witness. Reaching CONFIRMED requires cross-module linking
(architectural), per-module source dedup, and the mutability fixes -- tracked
here, not yet done.


## Compiler organ -- second codegen pass (2026-06-05, session continuation)

A second adjudication-driven pass landed five verified compiler fixes on the
quantalang branch fix/codegen-module-prefix. Each was diagnosed against the
compiler organ (tools/coherence/compiler_oracle.py), implemented, then verified
by rebuild plus the full compiler test suite (612 pass / 0 failed) and the
337-test codegen slice. Method discipline held throughout: every fix was
confirmed by re-adjudication (observe, never simulate), and the generated C was
read, not reasoned about.

Fixes (inner-repo commits on fix/codegen-module-prefix):
- e844bda -- Tuple typedefs were emitted twice (for a tuple that is both a
  struct-field type and a registered type definition); once de-duplicated, a
  tuple used by value was emitted after its user. The pre-emit step now skips
  already-declared tuples, and the topological emitter records a dependency on a
  tuple's own typedef. field-tensor's C2011 redefinition and the follow-on C2079
  ordering both clear.
- aab75e7 -- Impl methods returning a tuple of named structs never registered the
  tuple's type definition (only free functions did), so the C referenced an
  undeclared Tuple_RGB_RGB. Registration is now mirrored into both impl-method
  lowering paths. (Roadmap item 4: tuple-of-named-struct ordering. lumina and
  refract now emit and correctly order these tuples.)
- 9d387fe -- Indexing a Vec<T> typed every element as i32 (the element-type match
  lacked a Vec arm), selecting the wrong runtime getter and defeating string
  equality. foundation's QuantaString '==' (C2088) clears and now lowers to a
  string compare.
- 09ec158 -- Box/Rc/Arc and BTreeMap were not lowered as intrinsics and emitted
  undeclared C identifiers. Box/Rc/Arc now lower to a pointer and BTreeMap shares
  the map machinery. (Roadmap item 2, stdlib half. delta's Box/BTreeMap clear;
  oracle and entropy advance past Box. Honest caveat: BTreeMap aliases the
  unordered string-keyed map and Box<dyn Trait> becomes a thin pointer, so
  ordered/typed-key maps and trait-object dispatch remain separate gaps.)
- 9f69b60 -- The type checker treated reference mutability as invariant at
  argument positions, rejecting a &mut T argument where a &T parameter is
  expected. A call-boundary coercion now accepts that sound reborrow while unify
  stays invariant everywhere else. (Roadmap item 3. prism, neutrino, wavelength
  now transpile.)

Refreshed ground truth (adjudicate-all): all 16 modules remain CONTRADICTED, but
the blockers moved deeper -- each module carries several stacked defects, and one
layer was peeled per module. New first-blockers surfaced (field-tensor: a
function-pointer field syntax; prism/neutrino: a char-before-bool enum emission;
foundation: a map-iteration tuple assignment; oracle: dyn_Kernel; delta:
DirectionalBarrierType). "Errors moved deeper" is the witnessed evidence the
fixes landed; reaching CONFIRMED requires clearing the remaining layers and the
items below.

Remaining roadmap, by tractability:
- DUPLICATE SIBLING DEFINITIONS (roadmap item 1, plus the nexus failure). Five
  modules declare a top-level module name twice (spectrum: harmony, quantization;
  delta: market_making; oracle: changepoint, ensemble; quantum-finance:
  risk/oms/multi_asset/ml_signals; forge: tests), and nexus declares ModConflict
  and ConflictType twice. The frontend accepts duplicates and emits both bodies,
  producing the C redefinitions. The principled fix is a frontend rejection (as
  Rust does), but it is entangled: it turns the organism transpile gate red for
  the three registered files (spectrum, delta, oracle) and requires source
  de-duplication to restore green -- and the duplicates genuinely differ
  (spectrum's two harmony modules carry different variant sets), so they cannot be
  auto-merged. This needs an authoring decision; it is not a single clean compiler
  change. Held for direction.
- CROSS-MODULE AND FFI RESOLUTION (roadmap item 2, cross-module half). nova,
  calibrate, lumina, entangle, refract reference types defined in sibling modules
  (titan_color = spectrum, Vec2/Mat4 = foundation) or external C (refract:
  ID3D11RenderTargetView). Single-file compilation cannot resolve these; the clean
  path is a project/registry build (a Quanta.toml dependency graph and a
  module-name index) plus an opaque/extern type mechanism for FFI. Architectural;
  deferred. entangle's SnapshotId is undefined anywhere -- a source defect inside
  that envelope.

## Compiler organ -- third codegen pass (2026-06-05, seam continuation)

Three further codegen fixes on quantalang fix/codegen-module-prefix, each verified
by rebuild + the full suite (612 pass / 0 failed) and re-adjudication:
- 9a90491 -- enum unit-variant placeholder "char _<Variant>;" is escaped against
  reserved C identifiers (a variant named Bool produced _Bool, the C99 keyword;
  C2632). prism and neutrino clear the enum emission.
- a331f3f -- the C backend synthesizes typedefs for every tuple type referenced
  anywhere (parameters, locals, fields), not just literals and return types.
  Witnessed via cl-harvest: undefined Tuple_* identifiers 14 -> 0.
- 7b54733 -- const identifiers used as fixed-array lengths now resolve
  (try_const_eval gains an Ident/Path arm backed by a const_values pre-pass);
  [usize; MAX_DIMS] no longer emits an illegal zero-sized array.

Witnessed de-duplication inventory committed at docs/DEDUP-INVENTORY.md: 67
duplicate-sibling-definition families (classified IDENTICAL / SUPERSET / DIFFERENT)
plus the C-compiler-enumerated referenced-but-undefined set (7 SOURCE-PHANTOM, 25
CROSS-MODULE incl. 6 ambiguous, 3 FFI, and codegen-side gaps). This draws the seam:
source-authoring work (duplicate families, phantoms, cross-module ownership) on one
side; remaining compiler work (cross-module linking, FFI extern types, generic and
method emission) on the other.

## Generic/method emission -- investigation (2026-06-05, witnessed; NOT shipped)

Investigated the broad `Self*`-receiver leak (unresolved Self in emitted C across
12 modules: prism 237, refract 230, entangle 78, calibrate 36, neutrino 27,
oracle 26, field-tensor/nexus 20, ...). Findings, all witnessed:

- The Inventory-2 CODEGEN-fn bucket (ColorTransform_new, ScriptEngine_new, etc.)
  is CASCADE NOISE: those methods ARE emitted correctly; cl flagged them only
  because earlier undefined types poisoned their signature lines.
- The real defect: a PARSER bug. A generic method with an `Fn(...) -> T` bound
  (e.g. `fn map<F: Fn(f64) -> f64>(&self, f: F)`) is mis-parsed -- parse_path
  reads `Fn` and stops at `(`, leaving `(...) -> T` to derail the generic-param
  list and silently TRUNCATE the impl block. Every method after it parses as a
  top-level free function -> bare name + unresolved `Self*` receiver. Minimal
  repro: `impl S { fn make()->S{...} fn apply<F: Fn(f64)->f64>(&self,f:F)->S{...}
  fn doubled(&self)->S{...} }` emits `doubled(Self* self)` (top-level), not
  `S_doubled`.
- Fixing the parser (consume the Fn-sugar) is correct but UN-MASKS a cascade of
  unsupported generic-system features that the truncation was hiding:
  closure-type-param callability (`type F is not callable`), generic-method
  return inference (`function returns () but expected DefId`), method-on-
  type-param (`type T has no method shrink`). A parser+callability fix recovered
  3 modules (entropy/field-tensor/wavelength, leak reduced) and kept 612 tests
  green, but regressed 5 modules (oracle/prism/calibrate/neutrino/refract) from
  false-green transpile to honest transpile-failure.

Conclusion (truth-over-approval): completing generic/method emission is a major
generic-system feature spanning parser + AST (retain Fn-bound signature) +
typecheck (callability, return inference, associated-method resolution) +
method-with-closure monomorphization -- not a clean fix. The partial fix was
REVERTED to avoid a net regression (the organism transpile gate going red for 5
modules without delivering working generic methods). The false-green is itself a
truth gap: those modules "transpile" but emit broken C and use unsupported
features. Tracked here for a scoped feature effort.

## Generic/method emission -- core implemented (2026-06-06, quantalang 1641c97)

Supersedes the prior "investigation (NOT shipped)" note: the compiler-side core
is now implemented and committed (612 tests pass throughout):
- Parser: parse_type_bounds consumes Fn-trait sugar `Fn(A,B) -> C` (covers generic
  bounds and, via the shared bound parser, dyn/impl Trait). Fixes the silent impl
  truncation that leaked post-generic methods as free functions (bare name +
  unresolved Self* receiver).
- Typecheck: a call on a generic type parameter (or &/&mut of a param/var) is
  callable (fresh var, deferred to monomorphization); bodies of generic functions
  and of methods in a generic impl are no longer strict-checked abstractly.

Verified on clean code (repro: an impl with a closure-bounded generic method
followed by another method now emits Type_method(T* self) for both). Recovered to
correct transpile/emission: calibrate, neutrino, entropy, field-tensor (partial),
wavelength.

Current ground truth (adjudicate/transpile, 16 modules): 13 transpile, 3 fail. The
remaining tail is partly NOT compiler-fixable:
- oracle FAIL -- DefId<?T> collision in ConformalPredictor, caused by oracle's
  duplicate `mod`s (changepoint x2, ensemble x3). Source de-dup, not a compiler bug.
- entangle (78 residual Self*) -- MALFORMED SOURCE: a `pub fn route_task` is spliced
  into the middle of another method's `match` expression (a botched "// Stubbed"
  edit). Parser error-recovery leaks subsequent methods. Source defect.
- prism / refract FAIL -- `new()` returns `()` vs the struct type; a typecheck issue
  lowering struct fields of type Option<Box<dyn Fn(...) + Send + Sync>> (the field in
  isolation lowers fine, so it is a module-specific interaction). Compiler-side,
  needs deeper work.
- field-tensor (13 residual Self*) -- a module-specific impl truncation not yet
  pinned; its map<F: Fn(f64)->f64> is byte-identical to a working repro, so the
  trigger is elsewhere in that impl.

Conclusion: the generic/method-emission core works for clean code and is committed.
Full per-module completion is blocked by (a) source defects in oracle/entangle
(de-dup / malformed-stub cleanup -- source side) and (b) a long tail of
module-specific compiler edge-cases (prism/refract field lowering, field-tensor
truncation) that each need dedicated investigation. The committed milestone
intentionally trades 3 modules from false-green transpile (passing while emitting
broken C) to honest transpile-failure.

## Cross-module compilation -- oracle mechanism (2026-06-06, quanta-universe 05d8d7d)

The compiler organ can now compile a module WITH its transitive dependencies:
`python tools/coherence/compiler_oracle.py adjudicate-deps[-all]`. It builds a
module-name index (module <Name> decls across */lib.quanta, dir-name fallback),
resolves transitive `use <Mod>::...` deps, concatenates them ahead of the target,
and adjudicates the combined unit.

Ground truth (adjudicate-deps-all): cross-module linking RESOLVES the cross-module
type-reference class. Modules that pulled deps advanced from undefined-identifier
errors to deeper ones in the combined unit: calibrate XYZ -> "no field white";
nova Vec2 -> "no field title"; lumina Mat4 -> "no method scale"; wavelength ->
non-exhaustive match. No module compiles-with-deps yet -- residual blockers are
deeper type mismatches and the dependency modules' OWN source defects (e.g.
spectrum's duplicate mods). Modules with no resolvable cross-module import show
[+0 deps]; their blockers are source phantoms / duplicate definitions, not
cross-module. Mechanism validated: a clean dep+user concatenation emits C that cl
compiles (type-ordered). Kept oracle-side (observation); a compiler-side port
(quantac resolving cross-module directly) is deferred until the deeper issues and
dep source defects are cleared, since modules cannot compile-with-deps until then.

## Codegen: Vec-of-aggregate indexing + the lowering type-inference root (2026-06-06, 26aac3e)

Fixed (quantalang 26aac3e): indexing a Vec whose element is a struct/tuple selected
the scalar i32 getter; aggregate elements now use the runtime generic
quanta_vec_get(handle.inner, i) with cast+deref. Broad (any Vec<struct>/Vec<tuple>
index). 337 codegen tests pass. foundation advanced past its Vec<tuple> access.

Root-cause finding (foundation, a CLEAN module -- no dup-defs): its remaining
blockers are a deep chain of LOWERING type-inference defaults-to-i32. Next was
`let edges = <match/if with a map.get(node) branch>` -- the binding type defaulted
to i32 instead of the value type (Vec). Traced: the let-binding type is
type_of_value(init) (correct for direct calls), but for a CONDITIONAL/match init
the merged result local defaults to i32, so the binding mis-types. This is a
systemic weakness: the codegen lowering's own type inference (separate from the
typechecker) falls back to i32 in several places (binding types, conditional-result
merges, some collection-accessor returns). It is the recurring root behind the
clean modules' residual codegen failures. A typechecker-side .get fix was tried and
reverted -- ineffective, because the binding type comes from the lowering, not the
typechecker.

Assessment: per-module compilation bottoms out in either source defects (dup-defs /
malformed stubs -- source side) or this systemic lowering-inference weakness. The
highest-leverage remaining compiler work is hardening the lowering's type
propagation (reduce the i32 defaults), which is a sizable, higher-risk change to a
shared base -- not a clean per-site fix.

## Lowering type-inference hardening -- begun (2026-06-06, quantalang dfaf158, 97e8179)

Started driving out the codegen lowering's i32 defaults. New tool: infer_expr_type
-- a best-effort static AST-expression type inferrer (literals, idents, free
calls, collection method-calls map.get->V / vec.pop/first/last->T, struct
literals, self.field via pointer-peel, if/match/block tails). Applied at two
result-merge sites that previously guessed:
- lower_match result type: was the enclosing function's return type (enum) or the
  SCRUTINEE type otherwise; now inferred from the arm bodies. Verified:
  `let p = match n { _ => Pt {..} }` types p as Pt (was scrutinee i64), p.x
  compiles; `let p = match n { _ => self.pt }` resolves via the field type.
- Some(x)/TupleStruct match-arm binding: infers from the scrutinee expression
  when no tracked Option inner type exists, instead of the scrutinee local type.
612 tests pass; each fix verified cl-clean on a focused repro.

Note: this is real hardening but does NOT move the modules' single-module
first-blockers, which are dominated by other classes (cross-module types,
duplicate definitions, dyn_Fn, the Option/V mismatch). foundation specifically
bottoms out in an anomaly where `match self.edges.get(node) { Some(edges) => }`
types `edges` i32 even though the codegen emits the correct value-typed getter
(quanta_hmap_get_val_QuantaVecHandle) -- the get-result local is typed i32 while
the emitted getter knows it is a Vec, a contradiction that resists per-site fixing
and needs deeper tracing of the get-result local typing. Tracked, not yet cracked.

## Source-side de-duplication pass (2026-06-06)

Targeted the duplicate-sibling-definition blockers in spectrum, oracle, nexus.
- spectrum: the second `pub mod harmony` (4 colliding inner defs) renamed to
  harmony_palette -- conservative (no deletion); the harmony_HarmonyType C2365
  redefinition is gone. spectrum advances to a separate pre-existing codegen bug
  (C2059 '[' at module.c:3066). (quantization's two blocks are disjoint -> no
  collision -> left as harmless concatenation.)
- oracle: ConformalPredictor was defined twice -- a generic `<F: Fn..>` in
  mod conformal (def+impl only, never constructed) plus a non-generic top-level
  one with base_model. The generic's bare-name registration won resolution, so the
  non-generic construction resolved to DefId<?T> (no base_model). Renamed the
  unused generic to ConformalPredictorGeneric; oracle advances past 7512 to a
  separate source issue (undefined `relu`). (CompositeKernel enum/struct name
  clash remains latent, not the current blocker.)
- nexus: NOT de-duped. It carries two FULL parallel implementations of 10 types
  (ModConflict/ConflictType/ConflictSeverity/ModPackage/ProfileManager/
  GameDetector/VirtualFileSystem/AssetRedirector/ModDependency/ModFile), each with
  impls in BOTH clusters (e.g. impl ModPackage @199 and @4759), with different
  field types (cluster1 ConflictType vs cluster2 ModConflictType -- the latter
  undefined). Choosing which implementation is canonical is an authoring decision,
  not a safe mechanical de-dup; deferred.

Net: the de-dup cleared spectrum's and oracle's own duplicate-definition blockers
(both advance to separate, non-dup issues). It did NOT make any module compile or
immediately unblock the spectrum-dependents -- those fail earlier at transpile on
their own API mismatches (calibrate .white, nova .title, lumina .scale), unrelated
to the spectrum dup. nexus needs an operator decision on its two implementations.
