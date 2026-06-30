## Quanta Universe v1.0.2

This release publishes the refreshed module-forward tooling surface for split-repo packaging.

### Highlights
- Added generated release candidate dashboard: `releases/release-candidates.md`
- Added generated outward-facing module showcase: `tools/showcase.md`
- Extended `tools/release_plan.py` with `--write-showcase`
- Documented naming/conduct for module publish surfaces in `README.md` and `tools/README.md`
- Added git hygiene for tool cache artifacts (`tools/__pycache__`) in `.gitignore`

### Nested modules included (publish=true)
- QuantaOS, Axiom, Photon, Spectrum, Chromatic, Lumina, Nexus, Prism, Refract, Neutrino
- Quantum Finance, Field Tensor, Delta, Entropy, Entangle, Calibrate, Nova, Oracle, Wavelength, Forge, Foundation

This release is intended to support public-facing packaging workflows for nested repositories.
