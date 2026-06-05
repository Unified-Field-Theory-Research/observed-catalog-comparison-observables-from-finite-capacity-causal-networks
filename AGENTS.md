# Agent Instructions

## Paper 9 Target

This repository is organized around the ninth theorem-paper target:

> Observed-catalog comparison observables can be stated and conditionally
> recovered as network-native finite comparison-interface structure from the
> closed Paper 8 Standard-Model-candidate observable package while preserving
> finite capacity, locality, bounded transfer, causal-cone/no-signaling
> constraints, conservation/coarse-graining stability, and compatibility with
> the closed Papers 1 through 8 theorem chain, without importing observed
> particle catalog recovery, physical Standard Model content, physical
> particle excitations, continuum quantum field theory, background Hilbert
> bundles, external matter fields, external gauge fields, physical quantum
> dynamics, simulation-only promotion, fit-only calibration, or a unified
> field theory as microscopic premises.

Treat this repo as a downstream Paper 9 workspace. Do not edit the upstream
Paper 1, Paper 2, Paper 3, Paper 4, Paper 5, Paper 6, Paper 7, or Paper 8
repositories from this repo.

## Upstream Boundary

The upstream repositories are:

```text
https://github.com/Unified-Field-Theory-Research/finite-capacity-causal-geometry
https://github.com/Unified-Field-Theory-Research/higher-dimensional-geometry-recovery
https://github.com/Unified-Field-Theory-Research/curvature-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/gravitational-dynamics-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/quantum-compatible-local-dynamics-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/matter-gauge-observables-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/particle-excitation-observables-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/standard-model-candidate-observables-from-finite-capacity-causal-networks
```

The expected local sibling checkouts are `../finite-capacity-causal-geometry`,
`../higher-dimensional-geometry-recovery`,
`../curvature-from-finite-capacity-causal-networks`,
`../gravitational-dynamics-from-finite-capacity-causal-networks`,
`../quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`,
`../matter-gauge-observables-from-finite-capacity-causal-networks`,
`../particle-excitation-observables-from-finite-capacity-causal-networks`,
and
`../standard-model-candidate-observables-from-finite-capacity-causal-networks`.
Frozen upstream commits are recorded in `UPSTREAM-PAPERS.json`. Any Paper 9
artifact that depends on Papers 1 through 8 must cite the recorded commit or a
later explicitly approved upstream revision. If Paper 9 reveals a defect or
missing definition upstream, record an upstream-revision request instead of
silently modifying the upstream repo.

## Claim Boundary

This repo does not currently prove:

- observed-catalog comparison observable recovery;
- observed particle catalog recovery;
- physical Standard Model content;
- physical particle excitations;
- physical matter fields or physical gauge fields;
- physical quantum dynamics;
- continuum quantum field theory;
- physical/nature-level realization of the finite-capacity substrate;
- a unified field theory.

Closed rungs currently include only `OCC-001`: upstream binding through the
closed Paper 8 conditional Standard-Model-candidate observables theorem and a
claim-boundary scaffold. Observed-catalog recovery, physical Standard Model,
physical-particle, physical quantum-dynamics, continuum-QFT, physical-nature,
simulation-only, fit-only, and unified-field promotion flags remain false. Do
not promote this scaffold to observed particle catalog recovery, physical
Standard Model recovery, physical particle physics, physical quantum dynamics,
continuum QFT, nature-level realization, or unified-field promotion.

## Rust-Only Policy

Runtime, validators, tests, simulations, migration tooling, and proof-gate
helpers must be Rust-native. Do not add Python files, Python notebooks,
Python packaging, Python helper scripts, or Python dependency manifests.

Lean is permitted only as an opt-in proof-certificate lane under `GPD/formal`.
Do not make Lean mandatory for normal Rust tests unless explicitly scoped.

## Rigor Rule

Do not state that Paper 9 is proved until finite external-catalog descriptor
observables, finite comparison-map observables, Paper 8
Standard-Model-candidate compatibility, conservation/coarse-graining
stability, Paper 8 regime consistency, a no-hidden-observed-recovery/fit-only
import audit, and a final fail-closed certificate close in Lean or an
equivalent fail-closed audit, and the local Rust gates pass. Do not state that
nature uses this substrate unless a separate external physical-promotion track
closes.
