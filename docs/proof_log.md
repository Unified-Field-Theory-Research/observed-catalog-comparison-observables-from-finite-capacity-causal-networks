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

## OCC-002: Finite External-Catalog Descriptor Observable

Status: closed as a finite descriptor-row observable contract.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/observed_catalog_comparison_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/ObservedCatalogComparisonObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/observed_catalog_comparison_observables_gate.rs`

Rust anchors:

- `FiniteExternalCatalogDescriptorObservable`
- `FiniteExternalCatalogDescriptorObservable::canonical_occ002`
- `FiniteExternalCatalogDescriptorObservable::closes_occ002`
- `Paper9SkeletonCertificate::with_occ002_descriptor_closed`
- `occ002_finite_external_catalog_descriptor_marker`

Lean anchors:

- `OCC002FiniteExternalCatalogDescriptorObservableContract`
- `OCC002FiniteExternalCatalogDescriptorObservableContract.closed`
- `occ002_finite_external_catalog_descriptor_closed_from_fields`
- `occ002_missing_catalog_label_bound_not_closed`
- `occ002_missing_paper8_sector_rows_not_closed`
- `occ002_observed_particle_catalog_recovery_import_not_closed`
- `occ002_fit_only_calibration_not_closed`
- `occ002_canonical_finite_external_catalog_descriptor_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`OCC-002` defines finite external comparison-target descriptor rows with
finite catalog-label, observable-key, uncertainty/tolerance metadata,
provenance-tag, local comparison-domain, descriptor-readout, Paper 8
candidate-sector-family compatibility, Paper 8 candidate-interaction-family
compatibility, finite local capacity, and bounded-transfer gates. It does not
prove candidate-to-catalog comparison maps, observed-catalog comparison
observable recovery, observed particle catalog recovery, physical Standard
Model content, physical particle excitations, physical matter fields,
physical gauge fields, physical quantum dynamics, continuum quantum field
theory, simulation-only promotion, fit-only calibration, a physical
nature-level claim, or a unified field theory.

## OCC-003: Finite Candidate-To-Catalog Comparison Map

Status: closed as a finite comparison-map observable contract.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/observed_catalog_comparison_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/ObservedCatalogComparisonObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/observed_catalog_comparison_observables_gate.rs`

Rust anchors:

- `FiniteCandidateToCatalogComparisonMap`
- `FiniteCandidateToCatalogComparisonMap::canonical_occ003`
- `FiniteCandidateToCatalogComparisonMap::closes_occ003`
- `Paper9SkeletonCertificate::with_occ003_comparison_map_closed`
- `occ003_finite_candidate_to_catalog_comparison_map_marker`

Lean anchors:

- `OCC003FiniteCandidateToCatalogComparisonMapContract`
- `OCC003FiniteCandidateToCatalogComparisonMapContract.closed`
- `occ003_finite_candidate_to_catalog_comparison_map_closed_from_fields`
- `occ003_missing_occ002_descriptor_not_closed`
- `occ003_physical_identification_map_not_closed`
- `occ003_fit_only_calibration_not_closed`
- `occ003_canonical_finite_candidate_to_catalog_comparison_map_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`OCC-003` defines finite candidate-to-catalog comparison-map rows from Paper 8
candidate keys to finite `OCC-002` external descriptor keys. It requires
finite comparison-edge support, score/tolerance channel support, local
comparison neighborhoods, comparison readout boundaries, descriptor-support
compatibility, Paper 8 candidate-sector-family compatibility, Paper 8
candidate-interaction-family compatibility, finite capacity, and bounded
transfer. It rejects physical identification maps, fit-only calibration,
observed-catalog recovery imports, observed particle catalog recovery,
physical Standard Model content, physical particle excitations, physical
matter fields, physical gauge fields, physical quantum dynamics, continuum
quantum field theory, simulation-only promotion, physical promotion, and
unified-field promotion.

## OCC-004: Standard-Model-Candidate Compatibility

Status: closed as a Paper 8 compatibility contract.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/observed_catalog_comparison_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/ObservedCatalogComparisonObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/observed_catalog_comparison_observables_gate.rs`

Rust anchors:

- `StandardModelCandidateCompatibility`
- `StandardModelCandidateCompatibility::canonical_occ004`
- `StandardModelCandidateCompatibility::closes_occ004`
- `Paper9SkeletonCertificate::with_occ004_standard_model_candidate_compatibility_closed`
- `occ004_standard_model_candidate_compatibility_marker`

Lean anchors:

- `OCC004StandardModelCandidateCompatibilityContract`
- `OCC004StandardModelCandidateCompatibilityContract.closed`
- `occ004_standard_model_candidate_compatibility_closed_from_fields`
- `occ004_missing_paper8_certificate_not_closed`
- `occ004_missing_no_signaling_not_closed`
- `occ004_physical_standard_model_content_import_not_closed`
- `occ004_canonical_standard_model_candidate_compatibility_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`OCC-004` consumes the closed Paper 8 final Standard-Model-candidate
certificate and records that the `OCC-002` descriptor rows and `OCC-003`
comparison-map rows are compatible with Paper 8 candidate sector-family rows,
candidate interaction-family rows, conservation/coarse-graining rows, finite
capacity, locality, bounded transfer, and causal-cone/no-signaling
constraints. It rejects observed-catalog recovery imports, observed particle
catalog recovery, physical Standard Model content, physical particle
excitations, physical matter fields, physical gauge fields, physical quantum
dynamics, continuum quantum field theory, simulation-only promotion,
fit-only calibration, physical promotion, and unified-field promotion.

## Active Next Obligation

`OCC-005`: prove finite comparison stability and intrinsic coarse-graining
stability for descriptor and comparison-map rows while preserving
conservation, finite capacity, locality, bounded transfer, and
causal-cone/no-signaling constraints.
