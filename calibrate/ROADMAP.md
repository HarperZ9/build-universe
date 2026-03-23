# Calibrate Pro Roadmap

## Where We Are (v1.0)

A sensorless display calibration tool that works. Detects monitors, identifies panels from a 33-profile database, applies DDC-CI hardware corrections and 3D LUTs via dwm_lut, generates ICC profiles. Native-gamut calibration mode preserves wide gamut instead of clamping to sRGB. Honest about its limitations — all accuracy is predicted from panel databases, not measured.

**What works well:**
- Color science foundation (Oklab, JzAzBz, CAM16, PQ/HLG, ACES — all verified)
- One-command calibration with DWM LUT application
- Dual-metric verification (CIEDE2000 + CAM16-UCS)
- Exact gamut coverage via Sutherland-Hodgman polygon clipping
- BT.2390 EETF for HDR tone mapping
- ReShade/SpecialK PNG LUT export
- Native-gamut mode as default (doesn't compress wide gamut)

**What's honest:**
- All accuracy is labeled "predicted (sensorless)"
- No fabricated data — CCMs computed from primaries, not stored
- No neural networks despite the original "NeuralUX" name (renamed)

---

## Phase 1: Measurement-Backed Calibration (v1.1)

**Goal:** Bridge the gap between sensorless prediction and measured reality.

### 1.1 Colorimeter Integration
- [ ] Native USB HID driver for X-Rite i1Display Pro (most common in the community)
- [ ] Native USB HID driver for Datacolor SpyderX
- [ ] ArgyllCMS backend as fallback (spotread, dispread, colprof)
- [ ] Measurement workflow: display patch → read colorimeter → compute correction
- [ ] Per-unit panel characterization: measure YOUR panel's actual primaries, gamma, white point
- [ ] Store measured data alongside panel database predictions — show both

### 1.2 Verification Becomes Real
- [ ] Post-calibration re-measurement: display ColorChecker patches, measure each one
- [ ] Report MEASURED Delta E alongside predicted Delta E
- [ ] Grade based on measurements: "Measured: Reference Grade (dE 0.3)" vs "Predicted: Professional (dE 0.65)"
- [ ] Grayscale tracking measurement (21 steps, 0-100%)
- [ ] Gamma tracking chart from measurements

### 1.3 Hybrid Mode
- [ ] Sensorless calibration as starting point → refine with colorimeter
- [ ] Iterative: apply sensorless LUT, measure result, compute residual correction, apply refined LUT
- [ ] Convergence in 2-3 iterations to measured dE < 0.5

**Why this matters:** Every credible calibration tool (DisplayCAL, CalMAN, Lightspace) measures. Sensorless is our unique advantage for zero-effort calibration, but measured verification is what earns trust. Supporting both puts us in a category of our own.

---

## Phase 2: Windows HDR Pipeline (v1.2)

**Goal:** Become the definitive HDR calibration tool on Windows.

### 2.1 MHC2 ICC Profile Generation
- [ ] Generate ICC profiles with MHC2 (Windows HDR) color matrix tags
- [ ] Windows HDR calibration API integration (per-display HDR tone mapping)
- [ ] Auto-detect Windows HDR mode (on/off) and apply appropriate profile
- [ ] Handle the SDR-in-HDR white level (203 cd/m2 reference, configurable)

### 2.2 scRGB / Windows Composition
- [ ] Understand the DWM compositing pipeline: app → scRGB → display
- [ ] Generate corrections that work within the Windows HDR compositor
- [ ] Handle the scRGB → PQ conversion at the display output stage
- [ ] Test with both NVIDIA and AMD HDR output

### 2.3 HDR Content-Aware Calibration
- [ ] Detect content type: SDR, HDR10, HDR10+, Dolby Vision, HLG
- [ ] Apply appropriate tone mapping per content type
- [ ] MaxCLL/MaxFALL metadata handling for proper HDR10 display mapping
- [ ] BT.2446 Method A for scene-referred HDR-to-SDR conversion

### 2.4 Per-Application Color Management
- [ ] Detect which application is in focus
- [ ] Auto-switch LUT profiles (sRGB for browser, native for desktop, P3 for creative apps)
- [ ] Integration with Windows color management API for per-app ICC selection

**Why this matters:** HDR calibration on Windows is a wasteland. CalMAN charges $2000+ for HDR workflows. DisplayCAL doesn't support HDR. The HDR Discord community has been building ad-hoc solutions (dwm_lut, SpecialK, custom ReShade shaders). A proper tool that handles the full Windows HDR pipeline would be the first of its kind at any price point.

---

## Phase 3: Display-Specific Intelligence (v1.3)

**Goal:** Handle the real-world behavior that generic calibration tools ignore.

### 3.1 OLED ABL Compensation
- [ ] Model Auto Brightness Limiter behavior per panel (Samsung QD-OLED, LG WOLED)
- [ ] ABL triggers: full-screen white area %, sustained brightness, thermal
- [ ] Generate LUTs that account for ABL — don't target brightness the panel can't sustain
- [ ] Per-panel ABL curves from community measurements

### 3.2 Near-Black Handling
- [ ] QD-OLED near-black issues: raised blacks at low signal levels
- [ ] WOLED WRGB near-black color shift (green/magenta tint)
- [ ] VA panel black crush compensation
- [ ] Measure and correct the actual near-black gamma deviation

### 3.3 Uniformity Compensation
- [ ] Screen uniformity measurement (5x5 or 9x9 grid)
- [ ] Per-zone correction in 3D LUT (luminance and chrominance)
- [ ] Edge-to-center brightness falloff compensation
- [ ] Requires colorimeter for measurement pass

### 3.4 Color Volume (3D Gamut)
- [ ] Move beyond 2D gamut area (CIE xy) to 3D color volume (CIE Lab/Oklch)
- [ ] Gamut volume mapping accounts for luminance-dependent gamut changes
- [ ] OLED panels lose saturation at high luminance — model and compensate
- [ ] Report volume coverage alongside area coverage

### 3.5 Subpixel Layout Awareness
- [ ] QD-OLED triangle subpixel: affects text rendering, not color accuracy
- [ ] WOLED WRGB: white subpixel affects luminance calculations
- [ ] Inform users about subpixel-related rendering artifacts vs actual color issues

**Why this matters:** This is where Lilium, Ershin, and Marty live. They know their OLED's ABL curve, they've measured their near-black behavior, they understand why a 1000-nit peak spec doesn't mean 1000 nits sustained. A tool that models these display-specific behaviors — instead of treating every panel as an ideal colorimetric device — would earn immediate respect.

---

## Phase 4: Community Integration (v1.4)

**Goal:** Work with the tools and workflows the community already uses.

### 4.1 ReShade Deep Integration
- [ ] Generate ReShade .fx shaders, not just LUT PNGs
- [ ] Per-game calibration profiles
- [ ] Hook into ReShade's addon API for real-time LUT hot-swap
- [ ] Calibration overlay: show gamut warning, clipping visualization in-game

### 4.2 SpecialK Integration
- [ ] SpecialK HDR retrofit LUT generation (SDR game → HDR display)
- [ ] Integrate with SpecialK's HDR widget for live tone mapping control
- [ ] Auto-detect SpecialK installation and configure LUT paths

### 4.3 MadVR / mpv Integration
- [ ] Export .3dlut files for MadVR (madshi format)
- [ ] Generate mpv ICC/3DLUT configuration
- [ ] Handle MadVR's own tone mapping interaction (avoid double-correction)

### 4.4 OBS / Streaming
- [ ] OBS LUT filter export for color-accurate streaming
- [ ] Source-display to capture-card color space conversion LUTs
- [ ] Calibrate the streaming output separately from the display

### 4.5 Creative Application LUTs
- [ ] DaVinci Resolve project-level LUT generation
- [ ] Photoshop/Lightroom ICC profile installation wizard
- [ ] Blender OCIO configuration generation

**Why this matters:** Tools win by fitting into existing workflows, not by demanding users abandon them. The HDR gaming community already uses ReShade + SpecialK + dwm_lut. The video community uses MadVR + mpv. Meeting them where they are — with the calibration data they need in the format they use — is how you become indispensable.

---

## Phase 5: Continuous Calibration (v2.0)

**Goal:** Calibration that maintains itself over time.

### 5.1 Drift Detection
- [ ] Periodic re-measurement (weekly/monthly with colorimeter)
- [ ] Detect panel aging: OLED burn-in compensation, LED phosphor aging
- [ ] Alert when calibration has drifted beyond threshold

### 5.2 Ambient Light Adaptation
- [ ] Ambient light sensor integration (laptop sensors, USB sensors)
- [ ] Adapt white point and brightness to room lighting in real-time
- [ ] CAM16 viewing condition adjustment based on measured ambient

### 5.3 Multi-Display Matching
- [ ] Cross-display white point and brightness matching
- [ ] Ensure consistent appearance across different panel technologies
- [ ] Compensate for metamerism between display types (OLED vs LCD)

### 5.4 Panel Aging Model
- [ ] Track calibration history per panel over months/years
- [ ] Model luminance degradation and color shift from aging
- [ ] Predictive re-calibration scheduling based on usage patterns

**Why this matters:** Every other tool calibrates once and forgets. Displays change. Room lighting changes. A tool that maintains calibration continuously — adapting to the actual state of the display and environment — is what professional color management should have been from the start.

---

## Phase 6: Open Platform (v2.x)

**Goal:** Become the platform, not just the tool.

### 6.1 Community Panel Database
- [ ] Crowdsourced panel measurements (submit your colorimeter data)
- [ ] Per-unit variation statistics (how much do individual PG27UCDMs differ?)
- [ ] Community-verified vs factory-spec characterizations
- [ ] Web API for panel database queries

### 6.2 Plugin Architecture
- [ ] Python plugin API for custom calibration workflows
- [ ] Custom LUT generators (let researchers implement new gamut mapping algorithms)
- [ ] Custom measurement device drivers
- [ ] Custom output format exporters

### 6.3 Cross-Platform
- [ ] macOS: CoreDisplay calibration API, ColorSync ICC installation
- [ ] Linux: colord integration, X11/Wayland gamma ramp, ICC profile loading
- [ ] Keep Windows as the primary platform (most complex HDR pipeline)

### 6.4 Calibration Standard
- [ ] Publish the panel database format as an open standard
- [ ] Document the sensorless calibration methodology
- [ ] Provide reference implementations for verification
- [ ] Invite Rtings, TFTCentral, Hardware Unboxed to contribute measured data

**Why this matters:** DisplayCAL died because it was a one-person project with no community contribution mechanism. CalMAN survives on enterprise pricing. The opportunity is an open-platform calibration tool with a community-driven panel database — something that gets more accurate over time as more people contribute measurements.

---

## What We Don't Build

- **A general-purpose color management framework.** We calibrate displays. We don't replace ICC/OCIO/ACES pipelines.
- **A content grading tool.** We make your display accurate. DaVinci Resolve, Photoshop, and Blender do the creative work.
- **A TV calibration tool.** We focus on PC displays connected via DisplayPort/HDMI to a GPU. TV calibration has different workflows (service menus, Calman patterns via HDMI).
- **Hardware.** We don't make colorimeters. We integrate with existing ones.

---

## Competitive Landscape

| Tool | Strengths | Weaknesses | Our Advantage |
|------|-----------|------------|---------------|
| **DisplayCAL** | Open source, trusted, ArgyllCMS | Abandoned/forked, no HDR, ugly UI, Windows only gamma ramps | Modern architecture, HDR, sensorless mode, active development |
| **CalMAN** | Industry standard, comprehensive | $200-$2000+, enterprise-focused, no gaming features | Free/affordable, gaming-focused, community integration |
| **HCFR** | Free, measurement-focused | Ancient UI, limited, no LUT generation | Full pipeline from detection to LUT application |
| **dwm_lut** | System-wide 3D LUT on Windows | Manual .cube file workflow, no calibration | We generate the LUTs that dwm_lut applies |
| **SpecialK** | HDR retrofit, per-game | Not a calibration tool, manual tuning | We provide the calibration data SpecialK consumes |

**Our unique position:** The only tool that does sensorless zero-effort calibration AND supports measured calibration AND generates LUTs for the community's existing tools AND handles the Windows HDR pipeline. No other tool occupies this space.
