# QUANTA de-duplication inventory (witnessed from raw bytes)

## INVENTORY 1: duplicate sibling definitions

Classification: IDENTICAL (drop one) | SUPERSET (merge to the larger) |
DIFFERENT (disjoint members -> rename/namespace or hand-merge) | mod (the
duplicate is a module; COLLIDE lines are the inner defs that mangle to the
same C name).

### Summary

| module | definition | copies | classification |
|---|---|---|---|
| axiom | enum Term | 2 | DIFFERENT->rename/namespace or manual merge |
| axiom | struct Clause | 2 | DIFFERENT->rename/namespace or manual merge |
| config | enum ConfigError | 2 | SUPERSET->merge-to-superset |
| debug | enum StopReason | 2 | DIFFERENT->rename/namespace or manual merge |
| debug | struct MemoryPermissions | 2 | IDENTICAL |
| debug | struct MemoryRegion | 2 | SUPERSET->merge-to-superset |
| debug | struct MemorySnapshot | 2 | DIFFERENT->rename/namespace or manual merge |
| delta | mod market_making | 2 | mod (colliding inner: 0) |
| forge | mod tests | 2 | mod (colliding inner: 0) |
| forge | struct AllocationEvent | 2 | DIFFERENT->rename/namespace or manual merge |
| forge | struct Hotspot | 2 | DIFFERENT->rename/namespace or manual merge |
| forge | struct MemoryProfiler | 2 | DIFFERENT->rename/namespace or manual merge |
| lsp | enum SemanticTokenType | 2 | SUPERSET->merge-to-superset |
| lsp | struct Scope | 2 | DIFFERENT->rename/namespace or manual merge |
| lsp | struct SemanticToken | 2 | DIFFERENT->rename/namespace or manual merge |
| lsp | struct SemanticTokenModifiers | 2 | DIFFERENT->rename/namespace or manual merge |
| neutrino | struct QuantizationConfig | 2 | DIFFERENT->rename/namespace or manual merge |
| neutrino | struct QuantizedTensor | 2 | SUPERSET->merge-to-superset |
| nexus | enum ConflictSeverity | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | enum ConflictType | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct AssetRedirector | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct GameDetector | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct ModConflict | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct ModDependency | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct ModFile | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct ModPackage | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct ProfileManager | 2 | DIFFERENT->rename/namespace or manual merge |
| nexus | struct VirtualFileSystem | 2 | DIFFERENT->rename/namespace or manual merge |
| oracle | mod changepoint | 2 | mod (colliding inner: 0) |
| oracle | mod ensemble | 3 | mod (colliding inner: 0) |
| pkg | enum AnalyticsEventType | 2 | SUPERSET->merge-to-superset |
| pkg | enum DependencyType | 2 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct AnalyticsEvent | 2 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct PackageAnalytics | 2 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct PackageSigner | 2 | SUPERSET->merge-to-superset |
| pkg | struct RegistryConfig | 3 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct Signature | 2 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct SignatureVerifier | 2 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct VerificationResult | 3 | DIFFERENT->rename/namespace or manual merge |
| pkg | struct WorkspaceConfig | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | enum AuditEventType | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | enum AuditResult | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | enum ContainerState | 2 | SUPERSET->merge-to-superset |
| quantaos | enum CpuGovernor | 2 | IDENTICAL |
| quantaos | enum SeccompAction | 2 | SUPERSET->merge-to-superset |
| quantaos | enum SeccompOp | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct Container | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct ContainerConfig | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct ContainerRuntime | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct MountPoint | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct PowerManager | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct SeccompArg | 2 | IDENTICAL |
| quantaos | struct SeccompRule | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct SecurityLabel | 2 | DIFFERENT->rename/namespace or manual merge |
| quantaos | struct SecurityLevel | 2 | IDENTICAL |
| quantum-finance | mod ml_signals | 2 | mod (colliding inner: 2) |
| quantum-finance | mod multi_asset | 2 | mod (colliding inner: 5) |
| quantum-finance | mod oms | 2 | mod (colliding inner: 1) |
| quantum-finance | mod risk | 2 | mod (colliding inner: 0) |
| spectrum | mod harmony | 2 | mod (colliding inner: 4) |
| spectrum | mod quantization | 2 | mod (colliding inner: 0) |
| tests | enum MutationStatus | 2 | DIFFERENT->rename/namespace or manual merge |
| tests | struct CoverageData | 2 | DIFFERENT->rename/namespace or manual merge |
| tests | struct Mutation | 2 | DIFFERENT->rename/namespace or manual merge |
| tests | struct MutationTester | 2 | DIFFERENT->rename/namespace or manual merge |
| universe | struct Hypothesis | 2 | DIFFERENT->rename/namespace or manual merge |
| wavelength | struct SRTParser | 2 | IDENTICAL |

### Detail

## axiom :: struct Clause  x2  (L1351, 2641)  [DIFFERENT->rename/namespace or manual merge]
- L1351: literals
- L2641: body, head

## axiom :: enum Term  x2  (L1311, 2634)  [DIFFERENT->rename/namespace or manual merge]
- L1311: Const, Func, Var
- L2634: Compound, Constant, Variable

## config :: enum ConfigError  x2  (L679, 3472)  [SUPERSET->merge-to-superset]
- L679: IoError, NotFound, ParseError, ValidationError
- L3472: AuthenticationError, EncryptionError, IoError, NetworkError, NotFound, NotSupported, ParseError, RateLimitError, TimeoutError, ValidationError

## debug :: struct MemorySnapshot  x2  (L1979, 5121)  [DIFFERENT->rename/namespace or manual merge]
- L1979: MemoryPermissions, Vec, pages, permissions
- L5121: allocations, heap_usage, stack_usage, timestamp

## debug :: struct MemoryPermissions  x2  (L1985, 4740)  [IDENTICAL]
- L1985: execute, read, write
- L4740: execute, read, write

## debug :: struct MemoryRegion  x2  (L2386, 4733)  [SUPERSET->merge-to-superset]
- L2386: end, name, permissions, region_type, start
- L4733: end, name, permissions, start

## debug :: enum StopReason  x2  (L260, 4100)  [DIFFERENT->rename/namespace or manual merge]
- L260: Breakpoint, DataBreakpoint, Entry, Exception, FunctionBreakpoint, Goto, InstructionBreakpoint, Pause, Step
- L4100: Breakpoint, Exception, Exit, Pause, Signal, Step, Watchpoint

## delta :: mod market_making  x2  (L2494, 4650)
- copy L2494 items: struct OptionsMarketMaker, struct PositionInventory, struct TwoSidedQuote, struct RiskLimits, struct PricingParams, struct EdgeTargets, struct QuoteManager, struct QuoteEvent, enum QuoteEventType
- copy L4650 items: struct Quote, struct OptionPosition, struct InventoryState, struct MarketMakerConfig, struct MarketMaker, struct SpreadQuoter

## forge :: mod tests  x2  (L542, 9405)
- copy L542 items: (empty)
- copy L9405 items: fn test_logger, fn test_cli_parse, fn test_code_generator_struct, fn test_code_generator_enum, fn test_env_manager, fn test_pascal_case

## forge :: struct AllocationEvent  x2  (L4684, 5741)  [DIFFERENT->rename/namespace or manual merge]
- L4684: , address, false, is_alloc, size, thread_id, timestamp_ns
- L5741: address, event_type, size, timestamp

## forge :: struct Hotspot  x2  (L4730, 5673)  [DIFFERENT->rename/namespace or manual merge]
- L4730: file, function, line, percentage, sample_count
- L5673: file, line, percentage, time_ns

## forge :: struct MemoryProfiler  x2  (L4927, 5723)  [DIFFERENT->rename/namespace or manual merge]
- L4927: data, tracking
- L5723: AllocationRecord, allocation_history, allocations, current_memory, is_running, peak_memory

## lsp :: struct SemanticTokenModifiers  x2  (L1644, 5885)  [DIFFERENT->rename/namespace or manual merge]
- L1644: bits
- L5885: const

## lsp :: struct SemanticToken  x2  (L1678, 5909)  [DIFFERENT->rename/namespace or manual merge]
- L1678: delta_line, delta_start, length, modifiers, token_type
- L5909: length, line, modifiers, start_char, token_type

## lsp :: struct Scope  x2  (L1919, 6946)  [DIFFERENT->rename/namespace or manual merge]
- L1919: LocalSymbol, id, kind, parent, range, symbols
- L6946: expensive, name, variables_reference

## lsp :: enum SemanticTokenType  x2  (L1613, 5845)  [SUPERSET->merge-to-superset]
- L1613: Attribute, Class, Comment, Decorator, Enum, EnumMember, Event, Function, Interface, Keyword, Label, Lifetime, Macro, Method, Modifier, Namespace, Number, Operator, Parameter, Property, Regexp, SelfKeyword, String, Struct, Type, TypeParameter, Variable
- L5845: Class, Comment, Decorator, Enum, EnumMember, Event, Function, Interface, Keyword, Label, Macro, Method, Modifier, Namespace, Number, Operator, Parameter, Property, Regexp, String, Struct, Type, TypeParameter, Variable

## neutrino :: struct QuantizationConfig  x2  (L3794, 4084)  [DIFFERENT->rename/namespace or manual merge]
- L3794: activation_bits, calibration_method, per_channel, symmetric, weight_bits
- L4084: calibration_samples, per_channel, qtype, symmetric

## neutrino :: struct QuantizedTensor  x2  (L3806, 4086)  [SUPERSET->merge-to-superset]
- L3806: data, scale, shape, zero_point
- L4086: data, qtype, scale, shape, zero_point

## nexus :: struct ModDependency  x2  (L126, 4751)  [DIFFERENT->rename/namespace or manual merge]
- L126: max_version, min_version, mod_id, optional
- L4751: condition, mod_id, optional, version

## nexus :: struct ModFile  x2  (L181, 4753)  [DIFFERENT->rename/namespace or manual merge]
- L181: , file_type, hash, path, size, target_path
- L4753: checksum, condition, install_type, path, size, source

## nexus :: struct ModPackage  x2  (L191, 4682)  [DIFFERENT->rename/namespace or manual merge]
- L191: enabled, files, install_path, installed, metadata
- L4682: author, checksum, conflicts, created, dependencies, description, files, fomod_config, id, install_scripts, license, name, provides, size_bytes, updated, version, website

## nexus :: struct ModConflict  x2  (L325, 4388)  [DIFFERENT->rename/namespace or manual merge]
- L325: conflict_type, description, mod_a, mod_b, resolution, severity
- L4388: auto_resolvable, conflict_type, description, mod_a, mod_b, resource, severity

## nexus :: struct ProfileManager  x2  (L486, 4472)  [DIFFERENT->rename/namespace or manual merge]
- L486: ModProfile, active_profile, profiles
- L4472: SimpleProfile, active_profile, default_profile, profiles

## nexus :: struct GameDetector  x2  (L569, 5155)  [DIFFERENT->rename/namespace or manual merge]
- L569: signatures
- L5155: DetectedGame, detected, known_games, search_paths

## nexus :: struct VirtualFileSystem  x2  (L887, 4598)  [DIFFERENT->rename/namespace or manual merge]
- L887: Vec, backup_dir, hooks_installed, overlays
- L4598: String, case_sensitive, mount_points, overlays, read_only, root

## nexus :: struct AssetRedirector  x2  (L1512, 4546)  [DIFFERENT->rename/namespace or manual merge]
- L1512: String, Vec, redirects, stats, variables
- L4546: String, Vec, cache, enabled, redirects

## nexus :: enum ConflictType  x2  (L335, 4385)  [DIFFERENT->rename/namespace or manual merge]
- L335: , ExplicitIncompatibility
- L4385: AssetConflict, FileOverwrite, PluginConflict, RecordConflict, ResourceConflict

## nexus :: enum ConflictSeverity  x2  (L344, 4386)  [DIFFERENT->rename/namespace or manual merge]
- L344: , Info
- L4386: Critical, High, Low, Medium

## oracle :: mod changepoint  x2  (L2822, 7087)
- copy L2822 items: struct DetectedChangepoint, enum ChangeType, struct PELT, enum CostFunction, struct BOCPD
- copy L7087 items: enum ChangePointMethod, struct ChangePointDetector

## oracle :: mod ensemble  x3  (L5757, 9634, 9966)
- copy L5757 items: enum CombineStrategy, struct ForecastEnsemble, struct TimeSeriesBagging, struct ModelSelector, struct GradientBoostingForecaster, struct DecisionStump
- copy L9634 items: trait Forecaster, struct EnsembleForecaster, enum EnsembleCombinationMethod, struct EnsembleModelSelector, enum SelectionCriterion
- copy L9966 items: struct DecisionTree, struct TreeNode, struct RandomForest, struct GradientBoosting, struct XGBoostLite, struct AdaBoost, struct StackingEnsemble, struct VotingEnsemble, enum VotingType, struct BaggingRegressor

## pkg :: struct RegistryConfig  x3  (L191, 3365, 5305)  [DIFFERENT->rename/namespace or manual merge]
- L191: api_url, auth_token, cache_dir, download_url, index_url
- L3365: allowed_licenses, blocked_packages, max_package_size, mirror_from, require_signing, storage_path
- L5305: auth_token, registry_type, url

## pkg :: struct WorkspaceConfig  x2  (L1457, 3773)  [DIFFERENT->rename/namespace or manual merge]
- L1457: Dependency, default_members, dependencies, exclude, members, resolver
- L3773: , String, hoist_dependencies, link_protocol, packages

## pkg :: struct SignatureVerifier  x2  (L2847, 5029)  [DIFFERENT->rename/namespace or manual merge]
- L2847: PublicKey, config, trusted_keys
- L5029: VerifyingKey, require_signature, revoked_keys, trusted_keys

## pkg :: struct Signature  x2  (L2886, 3642)  [DIFFERENT->rename/namespace or manual merge]
- L2886: algorithm, data, key_id, timestamp
- L3642: algorithm, key_id, timestamp, value

## pkg :: struct VerificationResult  x3  (L2894, 3731, 5132)  [DIFFERENT->rename/namespace or manual merge]
- L2894: issues, package, signer, trust_level, verified, version
- L3731: error, key_id, owner, valid
- L5132: key_id, reason, valid, verified_at

## pkg :: struct PackageSigner  x2  (L3565, 4842)  [SUPERSET->merge-to-superset]
- L3565: algorithm, private_key, public_key
- L4842: algorithm, key_id, private_key, public_key

## pkg :: struct PackageAnalytics  x2  (L4414, 5144)  [DIFFERENT->rename/namespace or manual merge]
- L4414: aggregates, events
- L5144: enabled, endpoint, events, session_id

## pkg :: struct AnalyticsEvent  x2  (L4420, 5152)  [DIFFERENT->rename/namespace or manual merge]
- L4420: String, event_type, metadata, package, timestamp, version
- L5152: String, event_type, metadata, package_name, package_version, timestamp

## pkg :: enum DependencyType  x2  (L2574, 4222)  [DIFFERENT->rename/namespace or manual merge]
- L2574: Build, Dev, Direct, Transitive
- L4222: Dev, Direct, Optional, Peer, Transitive

## pkg :: enum AnalyticsEventType  x2  (L4429, 5161)  [SUPERSET->merge-to-superset]
- L4429: Build, Download, Install, Publish, Remove, Update
- L5161: Build, CacheHit, CacheMiss, Download, Error, Install, Publish, Remove, ResolutionComplete, ResolutionStart, Test, Update

## quantaos :: struct MountPoint  x2  (L1775, 4302)  [DIFFERENT->rename/namespace or manual merge]
- L1775: device, flags, fs_type, path, root_inode
- L4302: fs_type, options, read_only, source, target

## quantaos :: struct SecurityLabel  x2  (L3051, 5133)  [DIFFERENT->rename/namespace or manual merge]
- L3051: level, role, type_, user
- L5133: categories, label_type, level, role, user

## quantaos :: struct SecurityLevel  x2  (L3059, 5143)  [IDENTICAL]
- L3059: categories, sensitivity
- L5143: categories, sensitivity

## quantaos :: struct SeccompRule  x2  (L3080, 4428)  [DIFFERENT->rename/namespace or manual merge]
- L3080: action, args, syscall
- L4428: action, args, names

## quantaos :: struct SeccompArg  x2  (L3097, 4434)  [IDENTICAL]
- L3097: index, op, value
- L4434: index, op, value

## quantaos :: struct Container  x2  (L3280, 4218)  [DIFFERENT->rename/namespace or manual merge]
- L3280: cgroup, config, id, init_pid, name, namespaces, root_fs, state, u64
- L4218: String, cgroups, command, created_at, env, exit_code, id, image, mounts, name, namespaces, pid, resource_limits, security_context, started_at, state

## quantaos :: struct ContainerConfig  x2  (L3300, 4384)  [DIFFERENT->rename/namespace or manual merge]
- L3300: String, capabilities, cmd, cpu_quota, cpu_shares, env, hostname, memory_limit, network_mode, pids_limit, readonly_rootfs, seccomp, user, working_dir
- L4384: , cgroup_driver, default_runtime, default_ulimits, enable_apparmor, enable_seccomp, root_dir, state_dir

## quantaos :: struct ContainerRuntime  x2  (L3398, 4197)  [DIFFERENT->rename/namespace or manual merge]
- L3398: Cgroup, Container, Namespace, cgroups, containers, namespaces, next_ns_id
- L4197: Container, ContainerImage, ContainerNetwork, ContainerVolume, config, containers, images, networks, next_id, volumes

## quantaos :: struct PowerManager  x2  (L3576, 4753)  [DIFFERENT->rename/namespace or manual merge]
- L3576: CpuPState, ac_power, battery_level, cpu_states, governor, power_state, thermal_zones
- L4753: DevicePowerState, PowerPolicy, active_policy, cpu_states, device_states, events, policies, stats, suspend_handlers

## quantaos :: enum SeccompAction  x2  (L3087, 4420)  [SUPERSET->merge-to-superset]
- L3087: Allow, Errno, Kill, Log, Trace, Trap
- L4420: Allow, Errno, Kill, Log, Trap

## quantaos :: enum SeccompOp  x2  (L3104, 4441)  [DIFFERENT->rename/namespace or manual merge]
- L3104: Eq, Ge, Gt, Le, Lt, MaskedEq, Ne
- L4441: Equal, GreaterThan, LessThan, MaskedEqual, NotEqual

## quantaos :: enum AuditEventType  x2  (L3190, 5231)  [DIFFERENT->rename/namespace or manual merge]
- L3190: CapabilityUse, FileAccess, NetworkAccess, ProcessCreate, ProcessExit, SecurityViolation, Syscall
- L5231: AccessCheck, EnforcementChange, LabelChange, PolicyChange, PolicyLoad, SecurityViolation

## quantaos :: enum AuditResult  x2  (L3201, 5241)  [DIFFERENT->rename/namespace or manual merge]
- L3201: Denied, Failed, Success
- L5241: Denied, Error, Granted

## quantaos :: enum ContainerState  x2  (L3292, 4238)  [SUPERSET->merge-to-superset]
- L3292: Created, Paused, Running, Stopped
- L4238: Created, Error, Exited, Paused, Running, Starting, Stopped, Stopping

## quantaos :: enum CpuGovernor  x2  (L3545, 4823)  [IDENTICAL]
- L3545: Conservative, Ondemand, Performance, Powersave, Schedutil, Userspace
- L4823: Conservative, Ondemand, Performance, Powersave, Schedutil, Userspace

## quantum-finance :: mod risk  x2  (L1103, 9282)
- copy L1103 items: enum RiskLevel, enum PositionSizing, fn calculate_position_size, fn var_parametric, fn expected_shortfall, fn max_drawdown, fn sharpe_ratio, fn sortino_ratio
- copy L9282 items: struct RiskManager, struct Position, struct RiskCheckResult, struct PositionSizer, enum SizingMethod, struct CorrelationRisk

## quantum-finance :: mod oms  x2  (L1364, 9448)
- copy L1364 items: struct OrderIdGenerator, struct ValidationResult, struct OrderValidator, enum OrderEvent, trait OrderEventHandler, enum RoutingDestination, struct SmartOrderRouter, struct RoutingRule, struct FillTracker, struct Fill, enum LiquidityType, struct FillSummary, struct OrderManager
- copy L9448 items: struct OrderManagementSystem, struct Order, enum OrderSide, enum OrderType, enum OrderStatus, struct Fill, struct BracketOrder, struct OCOOrder, struct OrderRouter, struct Venue, enum RoutingMode
  * COLLIDE struct Fill [DIFFERENT->rename/namespace or manual merge]
    c1: commission, fill_id, liquidity, order_id, price, quantity, timestamp, venue
    c2: fee, order_id, price, quantity, timestamp

## quantum-finance :: mod multi_asset  x2  (L6569, 8733)
- copy L6569 items: enum AssetClass, struct Asset, struct MultiAssetPosition, struct MultiAssetPortfolio, struct RebalanceTrade, struct FactorModel, struct CurrencyHedger
- copy L8733 items: enum AssetClass, struct Asset, struct AssetAllocation, struct MultiAssetPortfolio, struct Holding, struct RebalanceTrade, struct PortfolioRiskMetrics, struct CurrencyHedger
  * COLLIDE enum AssetClass [DIFFERENT->rename/namespace or manual merge]
    c1: Alternative, Commodity, Crypto, Currency, Derivatives, Equity, FixedIncome, RealEstate
    c2: Commodity, Crypto, Currency, Derivative, Equity, FixedIncome, RealEstate
  * COLLIDE struct Asset [SUPERSET->merge-to-superset]
    c1: asset_class, currency, exchange, lot_size, margin_requirement, name, symbol, tick_size
    c2: asset_class, currency, lot_size, margin_requirement, name, symbol, tick_size
  * COLLIDE struct CurrencyHedger [DIFFERENT->rename/namespace or manual merge]
    c1: base_currency, f64, forward_rates, hedge_ratios
    c2: base_currency, f64, forward_points, hedge_ratios
  * COLLIDE struct MultiAssetPortfolio [DIFFERENT->rename/namespace or manual merge]
    c1: MultiAssetPosition, base_currency, cash_balances, f64, fx_rates, name, positions, target_allocations
    c2: Holding, allocations, base_currency, cash, f64, fx_rates, holdings, name
  * COLLIDE struct RebalanceTrade [IDENTICAL]
    c1: asset_class, current_value, target_value, trade_value
    c2: asset_class, current_value, target_value, trade_value

## quantum-finance :: mod ml_signals  x2  (L6828, 8963)
- copy L6828 items: enum Feature, struct FeatureVector, struct FeatureExtractor, struct FeatureConfig, enum FeatureType, struct SignalGenerator, enum Signal, struct EnsembleSignal, struct WalkForwardOptimizer, struct WalkForwardResult
- copy L8963 items: struct MLSignal, struct FeatureSet, struct Feature, enum FeatureCategory, struct FeatureExtractor, enum FeatureType, struct SignalCombiner, struct ModelWeight, enum CombinationMethod, struct SignalValidator, struct ValidationResult
  * COLLIDE enum FeatureType [DIFFERENT->rename/namespace or manual merge]
    c1: ATR, BollingerBand, EMA, MACD, Momentum, PricePosition, RSI, Returns, SMA, VolumeRatio
    c2: ATR, BollingerWidth, LogReturns, MACD, Momentum, PriceToMA, RSI, Returns, Skewness, Volatility, VolumeRatio
  * COLLIDE struct FeatureExtractor [DIFFERENT->rename/namespace or manual merge]
    c1: feature_config, lookback
    c2: features, lookback

## spectrum :: mod harmony  x2  (L2856, 6474)
- copy L2856 items: enum HarmonyType, struct HarmonyConfig, fn rgb_to_hsl, fn hsl_to_rgb, fn generate, fn harmony_score, fn suggest_scheme, fn accent_color
- copy L6474 items: enum HarmonyType, fn generate, fn calculate_harmony_score, fn rgb_to_hsl, fn hsl_to_rgb, struct PaletteGenerator, fn tint, fn shade, struct ColorPalette, fn relative_luminance
  * COLLIDE enum HarmonyType [SUPERSET->merge-to-superset]
    c1: Analogous, AnalogousTriad, Complementary, DoubleSplit, Monochromatic, SplitComplementary, Square, Tetradic, Triadic
    c2: Analogous, Complementary, Monochromatic, SplitComplementary, Square, Tetradic, Triadic
  * COLLIDE fn generate
  * COLLIDE fn hsl_to_rgb
  * COLLIDE fn rgb_to_hsl

## spectrum :: mod quantization  x2  (L3668, 5267)
- copy L3668 items: enum Algorithm, struct WeightedColor, struct Palette, struct ColorBox, fn median_cut, fn kmeans, fn kmeans_plusplus_init, fn dominant_colors
- copy L5267 items: struct ColorQuantizer, enum QuantizationMethod, struct Ditherer, enum DitheringMethod

## tests :: struct CoverageData  x2  (L1925, 4517)  [DIFFERENT->rename/namespace or manual merge]
- L1925: FileCoverage, covered_branches, covered_lines, files, total_branches, total_lines
- L4517: , HashSet, test_coverage

## tests :: struct MutationTester  x2  (L2435, 3404)  [DIFFERENT->rename/namespace or manual merge]
- L2435: killed, mutations, survived, timeout
- L3404: config, excluded_patterns, mutators, source_files, test_runner

## tests :: struct Mutation  x2  (L2443, 3431)  [DIFFERENT->rename/namespace or manual merge]
- L2443: file, id, kind, line, mutated, original, status
- L3431: column, description, file, id, line, mutated, mutator_name, original

## tests :: enum MutationStatus  x2  (L2468, 3452)  [DIFFERENT->rename/namespace or manual merge]
- L2468: CompileError, Killed, Pending, Survived, Timeout
- L3452: , Killed

## universe :: struct Hypothesis  x2  (L2830, 3729)  [DIFFERENT->rename/namespace or manual merge]
- L2830: is_assumption, name, prop
- L3729: assumptions, claim, created, domain, id, predictions, status, updated

## wavelength :: struct SRTParser  x2  (L5832, 7081)  [IDENTICAL]
- L5832: fn
- L7081: fn

## INVENTORY 2: referenced-but-undefined (C-compiler-witnessed, classified)

cl /c enumerates every undefined Capitalized identifier per module; each is
resolved against the repo symbol index and bucketed. Only SOURCE-PHANTOM and
CROSS-MODULE are de-duplication / resolution concerns; CODEGEN-* and
STD-INTRINSIC are compiler-side gaps (not source edits). FFI needs extern decls.

### SOURCE-PHANTOM (7)
- CVDResult  [ref: chromatic]  -> undefined type, defined nowhere
- ChromaticityCoords  [ref: calibrate]  -> undefined type, defined nowhere
- DirectionalBarrierType  [ref: delta]  -> undefined type, defined nowhere
- DynamicsProcessor  [ref: wavelength]  -> undefined type, defined nowhere
- HSL  [ref: chromatic]  -> undefined type, defined nowhere
- MonitorProfile  [ref: calibrate]  -> undefined type, defined nowhere
- SnapshotId  [ref: entangle]  -> undefined type, defined nowhere

### CROSS-MODULE (25)
- AmbientLightSensor  [ref: calibrate]  -> defined in calibrate
- AssetRedirectStats  [ref: nexus]  -> defined in nexus
- AtmosphericFog  [ref: lumina]  -> defined in lumina
- ColorGradeStyle  [ref: wavelength]  -> defined in wavelength
- ColorGrading  [ref: lumina]  -> defined in lumina, prism  AMBIGUOUS
- ColorPatch  [ref: calibrate]  -> defined in calibrate
- ColorSettings  [ref: nova]  -> defined in nova
- ConflictSeverity  [ref: nexus]  -> defined in nexus
- ConflictType  [ref: nexus]  -> defined in nexus
- ENBShaderConstants  [ref: refract]  -> defined in refract
- EffectsSettings  [ref: nova]  -> defined in nova
- Illuminant  [ref: calibrate]  -> defined in chromatic, spectrum  AMBIGUOUS
- InterpolationCurve  [ref: nova]  -> defined in nova
- Mat4  [ref: lumina, refract]  -> defined in foundation
- RGB  [ref: calibrate, lumina, nova, wavelength]  -> defined in spectrum
- RenderTargets  [ref: refract]  -> defined in refract
- RingBuffer  [ref: oracle]  -> defined in oracle, refract  AMBIGUOUS
- StructuralCausalModel  [ref: oracle]  -> defined in oracle
- ToneCurve  [ref: calibrate, chromatic]  -> defined in chromatic, spectrum  AMBIGUOUS
- TransactionState  [ref: entangle]  -> defined in entangle
- TypeId  [ref: entangle]  -> defined in runtime
- UniversalHook  [ref: nexus]  -> defined in photon
- Vec2  [ref: nova, prism]  -> defined in foundation, lumina  AMBIGUOUS
- Vec3  [ref: lumina, refract]  -> defined in field-tensor, foundation, neutrino, spectrum  AMBIGUOUS
- XYZ  [ref: calibrate]  -> defined in chromatic, spectrum  AMBIGUOUS

### FFI (3)
- ID3D11Device  [ref: refract]  -> external C/COM
- ID3D11RenderTargetView  [ref: refract]  -> external C/COM
- ID3D11Texture2D  [ref: refract]  -> external C/COM

### STD-INTRINSIC (2)
- Error  [ref: refract]  -> needs intrinsic lowering
- RwLock  [ref: delta]  -> needs intrinsic lowering

### CODEGEN-tuple (14)
- Tuple_DateTime_f64_f64  [ref: calibrate]  -> tuple typedef not emitted
- Tuple_FeaturePoint_bool  [ref: wavelength]  -> tuple typedef not emitted
- Tuple_Illuminant_Illuminant  [ref: chromatic]  -> tuple typedef not emitted
- Tuple_QuantaString_QuantaString  [ref: entropy]  -> tuple typedef not emitted
- Tuple_QuantaString_f64_u32  [ref: delta]  -> tuple typedef not emitted
- Tuple_QuantaString_unknown  [ref: nexus]  -> tuple typedef not emitted
- Tuple_RGB_f32  [ref: nova, spectrum]  -> tuple typedef not emitted
- Tuple_Tensor_usize_usize  [ref: neutrino]  -> tuple typedef not emitted
- Tuple_Version_Version  [ref: nexus]  -> tuple typedef not emitted
- Tuple_f64_usize  [ref: oracle]  -> tuple typedef not emitted
- Tuple_quantum_QuantumState_f64  [ref: field-tensor]  -> tuple typedef not emitted
- Tuple_u32_f32  [ref: wavelength]  -> tuple typedef not emitted
- Tuple_unknown_f64_f64  [ref: field-tensor]  -> tuple typedef not emitted
- Tuple_usize_usize  [ref: oracle]  -> tuple typedef not emitted

### CODEGEN-fn (8)
- ColorTransform_new  [ref: chromatic]  -> mangled Type_method not emitted
- FrameInterpolator_sample_bilinear  [ref: wavelength]  -> mangled Type_method not emitted
- ScriptEngine_new  [ref: nexus]  -> mangled Type_method not emitted
- ShaderHotReloader_new  [ref: prism]  -> mangled Type_method not emitted
- StabVideoStabilizer_bilinear_sample  [ref: wavelength]  -> mangled Type_method not emitted
- StabVideoStabilizer_handle_border  [ref: wavelength]  -> mangled Type_method not emitted
- VCSPresetVersionControl_new  [ref: nova]  -> mangled Type_method not emitted
- VideoFrame_get_pixel  [ref: wavelength]  -> mangled Type_method not emitted

### CODEGEN-vtable (2)
- EnginePlugin_vtable  [ref: nexus]  -> vtable instance not emitted
- Projection_vtable  [ref: entangle]  -> vtable instance not emitted

### CODEGEN-enumvariant (2)
- LookbackType_FixedStrike  [ref: delta]  -> mangled Enum_Variant not emitted
- LookbackType_FloatingStrike  [ref: delta]  -> mangled Enum_Variant not emitted

