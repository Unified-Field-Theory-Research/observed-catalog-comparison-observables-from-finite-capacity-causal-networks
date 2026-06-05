# Proof Log

## OCC-001: Upstream Binding And Claim Boundary

Status: closed as a scaffold-level upstream binding and nonpromotion contract.

Artifacts:

- `AGENTS.md`
- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/observed_catalog_comparison_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `GPD/formal/FiniteCapacity/ObservedCatalogComparisonObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/observed_catalog_comparison_observables_gate.rs`

Rust anchors:

- `Paper9UpstreamBinding`
- `Paper9UpstreamBinding::canonical_occ001`
- `Paper9UpstreamBinding::closes_occ001`
- `Paper9SkeletonCertificate::initial_occ001_only`
- `paper9_skeleton_marker`

Lean anchors:

- `OCC001UpstreamBindingContract`
- `OCC001UpstreamBindingContract.closed`
- `occ001_upstream_binding_closed_from_fields`
- `occ001_missing_paper8_standard_model_candidate_binding_not_closed`
- `occ001_observed_catalog_recovery_promotion_not_closed`
- `occ001_physical_standard_model_promotion_not_closed`
- `occ001_fit_only_calibration_not_closed`
- `occ001_unified_field_promotion_not_closed`
- `paper9_occ001_skeleton_does_not_close_observed_catalog_comparison_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`OCC-001` consumes the recorded Paper 8 final conditional
Standard-Model-candidate observables certificate and the recorded Paper 1
through Paper 7 chain. It does not prove observed-catalog comparison
observable recovery, observed particle catalog recovery, physical Standard
Model content, physical particle excitations, physical matter fields,
physical gauge fields, physical quantum dynamics, continuum quantum field
theory, simulation-only promotion, fit-only calibration, a physical
nature-level claim, or a unified field theory.

## Active Next Obligation

`OCC-002`: define a finite external-catalog descriptor observable without
importing observed particle catalog recovery, physical Standard Model content,
physical particle excitations, continuum QFT, external Hilbert bundles,
simulation-only promotion, fit-only calibration, physical quantum dynamics,
or unified-field promotion.
