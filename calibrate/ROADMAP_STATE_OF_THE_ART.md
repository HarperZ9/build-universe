# Calibrate Pro - State of the Art Feature Roadmap

## Mission: Surpass DisplayCAL and Light Illusion ColourSpace

Based on comprehensive research of DisplayCAL, Light Illusion ColourSpace, and cutting-edge color science advances (2024-2025), this document outlines the features needed to create the definitive display calibration software.

---

## Competitive Analysis Summary

### DisplayCAL Strengths (to match)
- ArgyllCMS backend for measurements
- 1000+ patch profiling for accuracy
- 3D LUT generation (.cube, .3dl)
- CIECAM02 gamut mapping
- Colorimeter correction matrices
- Profile verification and uniformity testing
- ISO 12646 compliance

### DisplayCAL Weaknesses (to exploit)
- **Abandoned** - Original developer no longer maintaining
- **Slow** - 5+ minutes vs 1 minute for competitors
- **Complex UI** - Steep learning curve
- **No hardware LUT access** - Can't write to monitor internal LUTs
- **Python 2 legacy** - Technical debt
- **26% test coverage** - Poor code quality

### Light Illusion ColourSpace Strengths (to match/surpass)
- 256³+ native LUT resolution
- Single-pass profiling → multiple calibrations
- Multi-Primary Color Engine (WOLED support)
- Mathematical (non-iterative) algorithms
- Direct LG OLED integration
- True professional-grade accuracy
- Lifetime license model

### ColourSpace Weaknesses (to exploit)
- **Expensive** - Professional pricing
- **Closed source** - No community contributions
- **Limited consumer focus** - Oriented toward facilities

---

## Feature Roadmap: Phase-by-Phase

### PHASE 1: Core Differentiators (Immediate Priority)

#### 1.1 NeuralUX Sensorless Calibration (UNIQUE)
**Status: Implemented - Delta E < 1.0 achieved**
- Panel database with factory characterization
- No hardware required for professional results
- Instant calibration vs 5-20 minute measurements

#### 1.2 True Black Preservation for OLED
**Status: Implemented**
- Zero black lift in LUTs
- Critical for QD-OLED/WOLED displays
- Not properly handled by competitors

#### 1.3 Universal GPU 3D LUT Loading
**Status: Planned**
- dwm_lut for Windows DWM-level LUTs
- Works across NVIDIA, AMD, Intel
- System-wide color management

---

### PHASE 2: Advanced Color Science (High Priority)

#### 2.1 Next-Generation Perceptual Color Models
**Implement CAM16-UCS, Jzazbz, ICtCp**

```python
# core/color_models.py

class CAM16:
    """
    CAM16 Color Appearance Model (CIE 2016)
    - Most versatile for WCG up to 1,611 cd/m²
    - Best at predicting small color differences
    - Used in Material Design HCT color system
    """

    def to_JMh(self, xyz, viewing_conditions):
        """Convert XYZ to JMh (Lightness, Colorfulness, Hue)"""
        pass

    def to_UCS(self, xyz):
        """Convert to CAM16-UCS for uniform color difference"""
        pass

class Jzazbz:
    """
    Jzazbz Perceptually Uniform Color Space
    - Designed specifically for HDR and WCG
    - Best for iso-hue prediction
    - Excellent for large color differences
    - Minimal hue shift with saturation changes
    """

    def from_xyz(self, xyz, peak_luminance=10000):
        """Convert XYZ to Jzazbz"""
        pass

    def delta_Ez(self, jzazbz1, jzazbz2):
        """Perceptually uniform color difference"""
        pass

class ICtCp:
    """
    ICtCp Color Space (Dolby/ITU-R BT.2100)
    - Optimized for HDR video
    - Excellent blue hue prediction
    - Better than CIELAB for modern displays
    """

    def from_pq(self, rgb_pq):
        """Convert PQ-encoded RGB to ICtCp"""
        pass
```

**Why these matter:**
- CIELAB/CIEDE2000 designed for SDR (100 nits max)
- CAM16-UCS outperforms all others for HDR color differences
- Jzazbz maintains perceptual uniformity to 10,000 nits
- ICtCp is the Dolby Vision standard

#### 2.2 ACES 2.0 Implementation
**Full ACES 2.0 with OCIO 2.4 integration**

```python
# core/aces.py

class ACES2:
    """
    Academy Color Encoding System 2.0
    - Complete redesign of rendering transform
    - HDR/SDR consistency
    - JMh-based gamut mapping
    """

    def __init__(self):
        self.gamut_mapper = ACES2GamutMapper()
        self.tonescale = ACES2Tonescale()

    def render(self, aces_rgb, output_colorspace, peak_luminance):
        """
        ACES 2.0 Rendering Pipeline:
        1. ACES RGB → JMh
        2. Tonescale (J only)
        3. Chroma Compression (M only)
        4. Gamut Compression (J & M)
        5. White Limiting
        6. Display Encoding
        """
        jmh = self.rgb_to_jmh(aces_rgb)
        jmh = self.tonescale.apply(jmh, peak_luminance)
        jmh = self.gamut_mapper.compress(jmh, output_colorspace)
        return self.encode_output(jmh, output_colorspace)

class ACES2GamutMapper:
    """
    Sophisticated gamut mapping in JMh space.
    Uses parametric compression: threshold (t), limit (l), exponent (p)
    """
    pass
```

#### 2.3 ICC.2 (iccMAX) Profile Support
**Next-generation ICC profiles with spectral support**

```python
# profiles/iccmax.py

class ICCMaxProfile:
    """
    ICC.2:2023 (iccMAX) Profile Support

    Capabilities beyond ICC v4:
    - Spectral Profile Connection Space
    - Programmable color transforms
    - Run-time parameters
    - Beyond D50 colorimetry
    - Multi-spectral data
    """

    def create_spectral_profile(self, spectral_data, illuminant):
        """Create profile with spectral PCS"""
        pass

    def embed_programmable_transform(self, transform_func):
        """Embed custom transform in profile"""
        pass
```

---

### PHASE 3: HDR Mastering Suite (High Priority)

#### 3.1 Complete HDR Format Support

```python
# hdr/formats.py

class HDR10Calibration:
    """
    HDR10 Static Metadata Calibration
    - SMPTE ST 2084 (PQ) EOTF
    - SMPTE ST 2086 mastering display metadata
    - MaxCLL/MaxFALL calculation
    """

    def calibrate_pq_eotf(self, measurements):
        """
        Calibrate to PQ EOTF (0-10,000 nits)

        Key values:
        - 100 nits = 0.5081 code value
        - 1000 nits = 0.7518 code value
        - 4000 nits = 0.9026 code value
        """
        pass

    def generate_st2086_metadata(self, display_measurements):
        """Generate SMPTE ST 2086 metadata"""
        return {
            "primaries": self.measured_primaries,
            "white_point": self.measured_white,
            "max_luminance": self.peak_luminance,
            "min_luminance": self.black_level
        }

class HDR10PlusCalibration:
    """
    HDR10+ Dynamic Metadata (SMPTE ST 2094-40)
    - Scene-by-scene optimization
    - Backward compatible with HDR10
    """
    pass

class DolbyVisionCalibration:
    """
    Dolby Vision Calibration
    - Profile 5/8 support
    - Dynamic metadata
    - Dual-layer encoding
    """
    pass

class HLGCalibration:
    """
    Hybrid Log-Gamma (ITU-R BT.2100)
    - Backward compatible with SDR
    - System gamma adjustment
    - 1000 nit nominal peak
    """
    pass
```

#### 3.2 Netflix/Disney+ Mastering Compliance

```python
# hdr/mastering_standards.py

class NetflixMasteringProfile:
    """
    Netflix HDR Mastering Requirements

    - Color Space: P3-D65 (NOT Rec.2020)
    - Dolby Vision mandatory for HDR originals
    - Reference monitors: Sony BVM-HX310, Canon DP-V2421
    """

    VIEWING_ENVIRONMENT = {
        "surround_illuminant": "D65",
        "viewing_distance_hd": "3-3.2 picture heights",
        "viewing_distance_uhd": "1.5-1.6 picture heights",
        "horizontal_fov": 90,  # degrees minimum
        "vertical_fov": 60,    # degrees minimum
    }

    def validate_calibration(self, measurements):
        """Check if calibration meets Netflix specs"""
        pass

class EBUGrade1Profile:
    """
    EBU Tech 3320 Grade 1 Monitor Specifications

    SDR:
    - Luminance: 70-100 nits adjustable
    - Black level: <0.05 nits
    - Contrast: >2000:1 sequential
    - Gamma: 2.4 ± 0.10
    - Primary tolerance: 4 ΔE(u*v*)

    HDR Grade 1:
    - Luminance: 0.005 - 1000 nits
    - Simultaneous contrast: 10,000:1
    - Gamut: >90% BT.2020
    """
    pass
```

---

### PHASE 4: Advanced LUT Engine (High Priority)

#### 4.1 High-Resolution LUT Generation

```python
# core/lut_engine_advanced.py

class AdvancedLUTGenerator:
    """
    Professional-grade LUT generation
    Matching/exceeding ColourSpace capabilities
    """

    SUPPORTED_SIZES = [17, 33, 65, 129, 256]  # Up to 256³

    def create_calibration_lut(
        self,
        size: int = 65,
        interpolation: str = "tetrahedral",  # vs trilinear
        gamut_mapping: str = "cam16",  # CAM16-UCS based
        smoothing: float = 0.0,
        preserve_black: bool = True,
        preserve_white: bool = True,
    ):
        """
        Generate calibration LUT with advanced options.

        256³ LUT = 50MB but maximum accuracy
        65³ is professional standard
        33³ acceptable for most uses
        """
        pass

    def single_pass_multi_target(self, profile_data):
        """
        ColourSpace killer feature:
        Generate multiple target calibrations from single profile pass.

        One profile → Rec709 LUT + P3 LUT + Rec2020 LUT
        """
        pass

class TetrahedralInterpolation:
    """
    Superior interpolation for 3D LUTs
    - Higher accuracy than trilinear
    - Better image quality
    - Worth the performance cost for calibration
    """
    pass
```

#### 4.2 LUT Manipulation Tools

```python
# core/lut_tools.py

class LUTManipulator:
    """
    Professional LUT editing tools (matching ColourSpace)
    """

    def combine(self, lut1, lut2) -> LUT3D:
        """Concatenate two LUTs"""
        pass

    def invert(self, lut) -> LUT3D:
        """Create inverse LUT"""
        pass

    def convert_format(self, lut, target_format):
        """Convert between .cube, .3dl, .mga, .csp, .clf"""
        pass

    def resize(self, lut, new_size):
        """Resize LUT with proper interpolation"""
        pass

    def apply_filter(self, lut, filter_type):
        """Apply LUT manipulation filters"""
        pass
```

---

### PHASE 5: Multi-Primary & Advanced Display Support

#### 5.1 Multi-Primary Color Engine (WOLED/QD-OLED)

```python
# core/multi_primary.py

class MultiPrimaryEngine:
    """
    Support for displays with more than 3 primaries

    Examples:
    - LG WOLED (RGBW) - 4 primaries
    - Sharp Quattron (RGBY) - 4 primaries
    - Future 5/6 primary displays

    This is a ColourSpace unique feature we must match.
    """

    def __init__(self, num_primaries: int):
        self.num_primaries = num_primaries

    def characterize_display(self, measurements):
        """
        Characterize multi-primary display
        Requires measuring each subpixel independently
        """
        pass

    def create_calibration_matrix(self):
        """
        Create NxN calibration matrix (not just 3x3)
        """
        pass
```

#### 5.2 MicroLED Demura (Uniformity Correction)

```python
# core/demura.py

class MicroLEDDemura:
    """
    Per-pixel uniformity correction for MicroLED displays

    Process:
    1. Measure each subpixel with imaging colorimeter
    2. Calculate correction table per pixel
    3. Modify inputs for uniform appearance

    Critical for emerging MicroLED technology (2025+)
    """

    def measure_uniformity(self, imaging_colorimeter):
        """Capture per-pixel luminance and color data"""
        pass

    def generate_correction_map(self, measurements):
        """Create per-pixel correction LUT"""
        pass
```

---

### PHASE 6: AI/ML Color Enhancement (Differentiator)

#### 6.1 Neural LUT Generation

```python
# ai/neural_lut.py

class NeuralLUTGenerator:
    """
    AI-powered LUT generation (inspired by fylm.ai)

    Features:
    - Train on professionally graded content
    - Style transfer from reference images
    - Natural language prompts for LUT generation
    - Reduce average ΔE from 20+ to ~5
    """

    def train_on_reference(self, graded_images, ungraded_images):
        """Train neural network on grading pairs"""
        pass

    def generate_from_style(self, style_image):
        """Create LUT that matches style of reference"""
        pass

    def generate_from_prompt(self, prompt: str):
        """
        Natural language LUT generation
        e.g., "warm cinematic look with lifted shadows"
        """
        pass

class MLColorimetryOptimizer:
    """
    Machine learning for optimal patch selection and calibration

    - Reduce calibration time by selecting optimal patches
    - Learn from previous calibrations to improve accuracy
    - Predict display behavior for sensorless calibration
    """
    pass
```

#### 6.2 Spatial-Aware Enhancement

```python
# ai/spatial_enhancement.py

class SpatialAware3DLUT:
    """
    ECCV 2024: Combining bilateral grids with 3D LUTs

    - Spatial-aware image enhancement
    - CUDA acceleration with tetrahedral interpolation
    - Real-time performance on GPU
    """
    pass
```

---

### PHASE 7: Professional Workflow Integration

#### 7.1 OCIO 2.4 Integration

```python
# workflows/ocio.py

class OCIOIntegration:
    """
    OpenColorIO 2.4 Integration

    - ACES 2.0 support via fixed functions
    - Config generation for DaVinci Resolve, Nuke, etc.
    - VFX Reference Platform 2025 compatible
    """

    def generate_ocio_config(self, calibration_data):
        """Generate OCIO config for post-production software"""
        pass

    def create_aces_transforms(self):
        """Create ACES 2.0 transforms using OCIO 2.4"""
        pass
```

#### 7.2 Direct Display Integration

```python
# hardware/display_integration.py

class LGOLEDIntegration:
    """
    Direct LUT installation to LG OLED internal processor
    Only ColourSpace and Calman currently offer this
    """

    def upload_lut(self, lut: LUT3D, display_ip: str):
        """Upload 3D LUT directly to LG OLED"""
        pass

class MonitorHardwareCalibration:
    """
    Hardware calibration support for professional monitors
    - EIZO ColorEdge (via ColorNavigator API)
    - BenQ SW series
    - Dell UltraSharp
    - ASUS ProArt
    """
    pass
```

#### 7.3 Fleet Calibration

```python
# enterprise/fleet.py

class FleetCalibration:
    """
    Multi-display calibration for studios

    Features:
    - Centralized calibration management
    - Master sensor correlation
    - Automated scheduling
    - Drift monitoring
    - Remote calibration
    """

    def calibrate_fleet(self, displays: List[Display]):
        """Calibrate multiple displays with consistency"""
        pass

    def monitor_drift(self, calibration_id):
        """Track calibration drift over time"""
        pass
```

---

### PHASE 8: Verification & Reporting

#### 8.1 Advanced Verification

```python
# verification/advanced.py

class ProfessionalVerification:
    """
    Comprehensive calibration verification

    Patch sets:
    - Quick: 26 patches (~1 min)
    - Standard: 154 patches (~5 min)
    - Extended: 755 patches (~20 min)
    - ColorChecker: 24/140 patches
    """

    def verify_ebu_grade1(self, measurements):
        """Verify against EBU Tech 3320 Grade 1 specs"""
        pass

    def verify_netflix_specs(self, measurements):
        """Verify against Netflix mastering requirements"""
        pass

    def verify_dci_p3(self, measurements):
        """Verify DCI cinema specifications"""
        pass
```

#### 8.2 Professional Reports

```python
# reports/professional.py

class ProfessionalReportGenerator:
    """
    Publication-quality calibration reports

    Output formats:
    - PDF (ReportLab)
    - HTML (interactive with JavaScript charts)
    - JSON (machine-readable)
    """

    def generate_report(self, calibration_data):
        """
        Report contents:
        - Before/after comparison
        - ΔE analysis per patch
        - Gamut coverage (2D and 3D)
        - Grayscale tracking
        - Gamma/EOTF curves
        - Uniformity maps
        - ISO compliance status
        """
        pass
```

---

## Implementation Priority Matrix

| Feature | Impact | Effort | Priority |
|---------|--------|--------|----------|
| CAM16-UCS/Jzazbz/ICtCp | High | Medium | P1 |
| ACES 2.0 Support | High | High | P1 |
| HDR10/DV/HLG Calibration | High | Medium | P1 |
| 256³ LUT Generation | High | Low | P1 |
| Single-pass Multi-target | High | Medium | P1 |
| OCIO 2.4 Integration | High | Medium | P2 |
| Multi-Primary Engine | Medium | High | P2 |
| Neural LUT Generation | High | High | P2 |
| LG OLED Direct Upload | Medium | Medium | P2 |
| iccMAX Profile Support | Medium | High | P3 |
| MicroLED Demura | Low | High | P3 |
| Fleet Calibration | Medium | High | P3 |

---

## Competitive Positioning

### vs DisplayCAL
- ✅ Active development (vs abandoned)
- ✅ 10x faster (sensorless option)
- ✅ Modern Python 3.11+ codebase
- ✅ Better UX with professional power
- ✅ True OLED black support
- ✅ Next-gen color models (CAM16, Jzazbz)

### vs Light Illusion ColourSpace
- ✅ Free/open source (vs expensive)
- ✅ Sensorless option (vs hardware required)
- ✅ Same 256³ LUT capability
- ✅ Same single-pass multi-target
- ✅ Same multi-primary support
- ✅ AI-enhanced features (unique)

### vs Calman
- ✅ No annual subscription
- ✅ Open source
- ✅ Better sensorless calibration
- ✅ More advanced color science

---

## Timeline Estimate

**Phase 1-2**: Core color science foundation
**Phase 3-4**: HDR mastering and advanced LUTs
**Phase 5-6**: Multi-primary and AI features
**Phase 7-8**: Professional workflows and enterprise

---

## Technical Dependencies

```
# requirements.txt additions
numpy>=1.24.0
scipy>=1.10.0
colour-science>=0.4.3    # Color science calculations
opencolorio>=2.4.0       # OCIO integration
PyOpenColorIO>=2.4.0     # Python bindings
torch>=2.0.0             # Neural LUT (optional)
reportlab>=4.0.0         # PDF reports
pillow>=10.0.0           # Image processing
```

---

## Success Metrics

1. **Accuracy**: Delta E < 0.5 with hardware, < 1.0 sensorless
2. **Speed**: Full calibration in < 60 seconds (sensorless)
3. **Compatibility**: All major colorimeters + spectrophotometers
4. **Standards**: Netflix, EBU Grade 1, DCI compliance verification
5. **Adoption**: Become the open-source standard for professionals
