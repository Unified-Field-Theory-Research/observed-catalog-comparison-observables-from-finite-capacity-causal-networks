# Observed-Catalog Comparison Observables From Finite-Capacity Causal Networks

This repository is the isolated Paper 9 workspace for the finite-capacity
emergent-spacetime program.

Paper 9 target:

> Starting from the closed Paper 8 conditional Standard-Model-candidate
> observables theorem, formulate finite network-native observed-catalog
> comparison observables while preserving finite capacity, locality, bounded
> transfer, causal-cone/no-signaling constraints, conservation/coarse-graining
> stability, and compatibility with the upstream geometry, curvature,
> dynamics, quantum-compatible local dynamics, matter/gauge observable,
> particle-excitation observable, and Standard-Model-candidate observable
> packages.

This repo is intentionally separate from the upstream theorem repositories:

- Paper 1/core repo: `https://github.com/Unified-Field-Theory-Research/finite-capacity-causal-geometry`
- Paper 2 geometry repo: `https://github.com/Unified-Field-Theory-Research/higher-dimensional-geometry-recovery`
- Paper 3 curvature repo: `https://github.com/Unified-Field-Theory-Research/curvature-from-finite-capacity-causal-networks`
- Paper 4 dynamics repo: `https://github.com/Unified-Field-Theory-Research/gravitational-dynamics-from-finite-capacity-causal-networks`
- Paper 5 quantum-compatible local dynamics repo: `https://github.com/Unified-Field-Theory-Research/quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`
- Paper 6 matter/gauge observables repo: `https://github.com/Unified-Field-Theory-Research/matter-gauge-observables-from-finite-capacity-causal-networks`
- Paper 7 particle-excitation observables repo: `https://github.com/Unified-Field-Theory-Research/particle-excitation-observables-from-finite-capacity-causal-networks`
- Paper 8 Standard-Model-candidate observables repo: `https://github.com/Unified-Field-Theory-Research/standard-model-candidate-observables-from-finite-capacity-causal-networks`
- Expected local sibling checkouts: `../finite-capacity-causal-geometry`,
  `../higher-dimensional-geometry-recovery`,
  `../curvature-from-finite-capacity-causal-networks`,
  `../gravitational-dynamics-from-finite-capacity-causal-networks`,
  `../quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`,
  `../matter-gauge-observables-from-finite-capacity-causal-networks`,
  `../particle-excitation-observables-from-finite-capacity-causal-networks`,
  and
  `../standard-model-candidate-observables-from-finite-capacity-causal-networks`
- Upstream binding file: `UPSTREAM-PAPERS.json`

The current theorem package has closed `OCC-001` through `OCC-008`. `OCC-001`
records the upstream binding and claim boundary through the closed Paper 8
result. `OCC-002` defines finite external-catalog descriptor observables with
bounded labels, observable keys, tolerance metadata, provenance tags, local
comparison domains, descriptor readout boundaries, Paper 8 candidate-row
compatibility, finite local capacity, and bounded transfer. `OCC-003` defines
finite candidate-to-catalog comparison-map rows with bounded candidate keys,
descriptor keys, comparison edges, score/tolerance channels, local
neighborhoods, readout boundaries, and no physical-identification or fit-only
shortcuts. `OCC-004` records compatibility with the closed Paper 8
Standard-Model-candidate candidate-sector, interaction-family,
conservation/coarse-graining, finite-capacity, locality, bounded-transfer,
and causal-cone/no-signaling rows. `OCC-005` records finite stability and
non-growing intrinsic coarse-graining for descriptor and comparison-map rows.
`OCC-006` binds the closed rows to the recorded Paper 8 regime and rejects
upstream bypasses or unapproved revisions. `OCC-007` closes the no-hidden
observed-recovery/fit-only import audit across docs, state, manifests, Lean,
Rust, publication metadata, and Rust-only runtime gates. `OCC-008` closes the
final internal conditional Paper 9 observed-catalog comparison observables
theorem. This does not prove observed particle catalog recovery, physical
Standard Model content, physical particle
excitations, physical matter fields, physical gauge fields, physical quantum
dynamics, continuum quantum field theory, simulation-only promotion, fit-only
calibration, physical nature realization, or a unified field theory.

Persistent boundary tokens: observed particle catalog recovery; physical Standard Model
content; physical particle excitations; physical matter fields;
physical gauge fields; physical quantum dynamics; continuum quantum field theory;
simulation-only promotion; fit-only calibration; unified-field promotion.

## Open Source, Authorship, And Citation

Primary author: Abraham Greenman.

- [LICENSE](LICENSE): code/tooling under MIT; manuscripts, diagrams,
  documentation, and review packets under CC BY 4.0 unless a file states
  otherwise.
- [CITATION.cff](CITATION.cff): citation metadata for this Paper 9 skeleton.
- [OPEN_SOURCE.md](OPEN_SOURCE.md): provenance, credit, and non-promotion
  framing.

## Layout

- `GPD/publication/observed-catalog-comparison-observables-theorem/manuscript/`:
  Paper 9 manuscript skeleton and publication metadata.
- `GPD/formal/FiniteCapacity/ObservedCatalogComparisonObservables.lean`:
  Lean scaffold for Paper 9 gates and fail-closed blockers.
- `docs/observed_catalog_comparison_observables_theorem.md`:
  theorem target, `OCC-001`, planned future rungs, and residual promotion
  obligations.
- `rust/cclab_accel/tests/observed_catalog_comparison_observables_gate.rs`:
  Rust guard that keeps the skeleton non-promoting and bound to Papers 1-8.

## Checks

```bash
make test-fast
make lean-build
```
