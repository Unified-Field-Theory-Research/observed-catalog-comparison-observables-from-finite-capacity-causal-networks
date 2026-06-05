use cclab_accel::{
    occ002_finite_external_catalog_descriptor_marker,
    occ003_finite_candidate_to_catalog_comparison_map_marker, paper9_skeleton_marker,
    FiniteCandidateToCatalogComparisonMap, FiniteExternalCatalogDescriptorObservable,
    Paper9SkeletonCertificate, Paper9UpstreamBinding, PAPER1_FROZEN_COMMIT, PAPER2_FROZEN_COMMIT,
    PAPER3_FROZEN_COMMIT, PAPER4_FROZEN_COMMIT, PAPER5_FROZEN_COMMIT, PAPER6_FROZEN_COMMIT,
    PAPER7_FROZEN_COMMIT, PAPER8_FINAL_CERTIFICATE, PAPER8_FROZEN_COMMIT,
};
use std::fs;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate remains under rust/cclab_accel")
        .to_path_buf()
}

fn read(root: &Path, relative: &str) -> String {
    let path = root.join(relative);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("can read {}: {err}", path.display()))
}

fn assert_contains(text: &str, token: &str, artifact: &str) {
    assert!(
        text.contains(token),
        "{artifact} is missing required token {token:?}"
    );
}

fn collect_python_artifacts(path: &Path, out: &mut Vec<PathBuf>) {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    if matches!(file_name, ".git" | "target" | ".lake") {
        return;
    }

    let metadata = fs::symlink_metadata(path).expect("can stat repository path");
    if metadata.is_dir() {
        if file_name == "__pycache__" {
            out.push(path.to_path_buf());
            return;
        }
        for entry in fs::read_dir(path).expect("can read repository directory") {
            let entry = entry.expect("can read repository directory entry");
            collect_python_artifacts(&entry.path(), out);
        }
        return;
    }

    if matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("py" | "pyc" | "pyo")
    ) {
        out.push(path.to_path_buf());
    }
}

#[test]
fn paper9_skeleton_files_exist() {
    let root = project_root();
    for relative in [
        "AGENTS.md",
        "README.md",
        "UPSTREAM-PAPERS.json",
        "GPD/PROJECT.md",
        "GPD/ROADMAP.md",
        "GPD/STATE.md",
        "GPD/state.json",
        "docs/observed_catalog_comparison_observables_theorem.md",
        "docs/open_proof_obligations.md",
        "docs/proof_log.md",
        "GPD/formal/FiniteCapacity/ObservedCatalogComparisonObservables.lean",
        "GPD/publication/observed-catalog-comparison-observables-theorem/manuscript/observed_catalog_comparison_observables_theorem.tex",
        "GPD/publication/observed-catalog-comparison-observables-theorem/manuscript/PAPER-CONFIG.json",
        "GPD/publication/observed-catalog-comparison-observables-theorem/manuscript/ARTIFACT-MANIFEST.json",
        "GPD/publication/observed-catalog-comparison-observables-theorem/manuscript/reproducibility-manifest.json",
    ] {
        assert!(
            root.join(relative).exists(),
            "missing required file {relative}"
        );
    }
}

#[test]
fn occ001_upstream_binding_consumes_closed_paper8_chain() {
    let binding = Paper9UpstreamBinding::canonical_occ001();

    assert_eq!(binding.paper1_commit, PAPER1_FROZEN_COMMIT);
    assert_eq!(binding.paper2_commit, PAPER2_FROZEN_COMMIT);
    assert_eq!(binding.paper3_commit, PAPER3_FROZEN_COMMIT);
    assert_eq!(binding.paper4_commit, PAPER4_FROZEN_COMMIT);
    assert_eq!(binding.paper5_commit, PAPER5_FROZEN_COMMIT);
    assert_eq!(binding.paper6_commit, PAPER6_FROZEN_COMMIT);
    assert_eq!(binding.paper7_commit, PAPER7_FROZEN_COMMIT);
    assert_eq!(binding.paper8_commit, PAPER8_FROZEN_COMMIT);
    assert!(binding.paper8_final_certificate_consumed);
    assert!(binding.closes_occ001());

    let promoted_comparison = Paper9UpstreamBinding {
        observed_catalog_comparison_observable_recovery_claim: true,
        ..binding
    };
    assert!(!promoted_comparison.closes_occ001());

    let promoted_observed_recovery = Paper9UpstreamBinding {
        observed_particle_catalog_recovery_claim: true,
        ..binding
    };
    assert!(!promoted_observed_recovery.closes_occ001());

    let promoted_fit_only = Paper9UpstreamBinding {
        fit_only_calibration_claim: true,
        ..binding
    };
    assert!(!promoted_fit_only.closes_occ001());

    let wrong_paper8_commit = Paper9UpstreamBinding {
        paper8_commit: "unapproved-paper8-revision",
        ..binding
    };
    assert!(!wrong_paper8_commit.closes_occ001());

    let mutable_upstream = Paper9UpstreamBinding {
        may_modify_upstreams: true,
        ..binding
    };
    assert!(!mutable_upstream.closes_occ001());
}

#[test]
fn initial_skeleton_does_not_close_paper9_theorem() {
    let certificate = Paper9SkeletonCertificate::initial_occ001_only();

    assert!(certificate.occ001_upstream_binding_closed);
    assert!(!certificate.occ002_finite_external_catalog_descriptor_closed);
    assert!(!certificate.occ008_final_conditional_certificate_closed);
    assert!(!certificate.paper9_theorem_closed);
    assert!(!certificate.closes_paper9_theorem());
    assert_eq!(
        paper9_skeleton_marker(),
        "paper9-observed-catalog-comparison-observables-occ001-nonpromoting-skeleton"
    );
}

#[test]
fn occ002_finite_external_catalog_descriptor_closes_only_descriptor_rung() {
    let descriptor = FiniteExternalCatalogDescriptorObservable::canonical_occ002();

    assert!(descriptor.occ001_upstream_binding_closed);
    assert!(descriptor.closes_occ002());
    assert!(descriptor.occupied_catalog_label_count <= descriptor.catalog_label_bound);
    assert!(descriptor.occupied_observable_key_count <= descriptor.observable_key_bound);
    assert!(descriptor.occupied_tolerance_metadata_count <= descriptor.tolerance_metadata_bound);
    assert!(descriptor.occupied_provenance_tag_count <= descriptor.provenance_tag_bound);
    assert!(descriptor.local_comparison_domain_size <= descriptor.finite_capacity_bound);
    assert!(descriptor.descriptor_readout_boundary_size <= descriptor.local_comparison_domain_size);
    assert!(descriptor.bounded_transfer_bound <= descriptor.finite_capacity_bound);
    assert!(descriptor.paper8_candidate_sector_family_rows_compatible);
    assert!(descriptor.paper8_candidate_interaction_family_rows_compatible);
    assert!(descriptor.finite_local_capacity_compatible);
    assert!(descriptor.bounded_transfer_compatible);
    assert!(!descriptor.observed_particle_catalog_recovery_import);
    assert!(!descriptor.physical_standard_model_content_import);
    assert!(!descriptor.physical_particle_excitation_import);
    assert!(!descriptor.continuum_qft_import);
    assert_eq!(
        occ002_finite_external_catalog_descriptor_marker(),
        "occ002-finite-external-catalog-descriptor-observable-closed"
    );

    let certificate = Paper9SkeletonCertificate::with_occ002_descriptor_closed();
    assert!(certificate.occ001_upstream_binding_closed);
    assert!(certificate.occ002_finite_external_catalog_descriptor_closed);
    assert!(!certificate.occ003_finite_candidate_to_catalog_comparison_map_closed);
    assert!(!certificate.occ008_final_conditional_certificate_closed);
    assert!(!certificate.paper9_theorem_closed);
    assert!(!certificate.closes_paper9_theorem());
}

#[test]
fn occ002_descriptor_fails_closed_on_nonfinite_or_imported_structure() {
    let descriptor = FiniteExternalCatalogDescriptorObservable::canonical_occ002();

    let missing_occ001 = FiniteExternalCatalogDescriptorObservable {
        occ001_upstream_binding_closed: false,
        ..descriptor
    };
    assert!(!missing_occ001.closes_occ002());

    let zero_catalog_bound = FiniteExternalCatalogDescriptorObservable {
        catalog_label_bound: 0,
        ..descriptor
    };
    assert!(!zero_catalog_bound.closes_occ002());

    let label_count_exceeds_bound = FiniteExternalCatalogDescriptorObservable {
        occupied_catalog_label_count: descriptor.catalog_label_bound + 1,
        ..descriptor
    };
    assert!(!label_count_exceeds_bound.closes_occ002());

    let readout_exceeds_domain = FiniteExternalCatalogDescriptorObservable {
        descriptor_readout_boundary_size: descriptor.local_comparison_domain_size + 1,
        ..descriptor
    };
    assert!(!readout_exceeds_domain.closes_occ002());

    let missing_paper8_sector_rows = FiniteExternalCatalogDescriptorObservable {
        paper8_candidate_sector_family_rows_compatible: false,
        ..descriptor
    };
    assert!(!missing_paper8_sector_rows.closes_occ002());

    let missing_paper8_interaction_rows = FiniteExternalCatalogDescriptorObservable {
        paper8_candidate_interaction_family_rows_compatible: false,
        ..descriptor
    };
    assert!(!missing_paper8_interaction_rows.closes_occ002());

    let hidden_observed_recovery = FiniteExternalCatalogDescriptorObservable {
        observed_particle_catalog_recovery_import: true,
        ..descriptor
    };
    assert!(!hidden_observed_recovery.closes_occ002());

    let hidden_physical_standard_model = FiniteExternalCatalogDescriptorObservable {
        physical_standard_model_content_import: true,
        ..descriptor
    };
    assert!(!hidden_physical_standard_model.closes_occ002());

    let hidden_physical_particle = FiniteExternalCatalogDescriptorObservable {
        physical_particle_excitation_import: true,
        ..descriptor
    };
    assert!(!hidden_physical_particle.closes_occ002());

    let fit_only = FiniteExternalCatalogDescriptorObservable {
        fit_only_calibration: true,
        ..descriptor
    };
    assert!(!fit_only.closes_occ002());
}

#[test]
fn occ003_finite_candidate_to_catalog_comparison_map_closes_only_map_rung() {
    let comparison_map = FiniteCandidateToCatalogComparisonMap::canonical_occ003();

    assert!(comparison_map.occ001_upstream_binding_closed);
    assert!(comparison_map.occ002_finite_external_catalog_descriptor_closed);
    assert!(comparison_map.closes_occ003());
    assert!(
        comparison_map.occupied_candidate_row_key_count <= comparison_map.candidate_row_key_bound
    );
    assert!(
        comparison_map.occupied_descriptor_row_key_count <= comparison_map.descriptor_row_key_bound
    );
    assert!(comparison_map.occupied_comparison_edge_count <= comparison_map.comparison_edge_bound);
    assert!(comparison_map.occupied_score_channel_count <= comparison_map.score_channel_bound);
    assert!(
        comparison_map.local_comparison_neighborhood_size <= comparison_map.finite_capacity_bound
    );
    assert!(
        comparison_map.comparison_readout_boundary_size
            <= comparison_map.local_comparison_neighborhood_size
    );
    assert!(comparison_map.bounded_transfer_bound <= comparison_map.finite_capacity_bound);
    assert!(comparison_map.descriptor_support_compatible);
    assert!(comparison_map.paper8_candidate_sector_family_rows_compatible);
    assert!(comparison_map.paper8_candidate_interaction_family_rows_compatible);
    assert!(comparison_map.finite_capacity_compatible);
    assert!(comparison_map.bounded_transfer_compatible);
    assert!(!comparison_map.physical_identification_map);
    assert!(!comparison_map.fit_only_calibration);
    assert!(!comparison_map.observed_catalog_recovery_import);
    assert!(!comparison_map.observed_particle_catalog_recovery_import);
    assert!(!comparison_map.physical_standard_model_content_import);
    assert!(!comparison_map.continuum_qft_import);
    assert_eq!(
        occ003_finite_candidate_to_catalog_comparison_map_marker(),
        "occ003-finite-candidate-to-catalog-comparison-map-closed"
    );

    let certificate = Paper9SkeletonCertificate::with_occ003_comparison_map_closed();
    assert!(certificate.occ001_upstream_binding_closed);
    assert!(certificate.occ002_finite_external_catalog_descriptor_closed);
    assert!(certificate.occ003_finite_candidate_to_catalog_comparison_map_closed);
    assert!(!certificate.occ004_standard_model_candidate_compatibility_closed);
    assert!(!certificate.occ008_final_conditional_certificate_closed);
    assert!(!certificate.paper9_theorem_closed);
    assert!(!certificate.closes_paper9_theorem());
}

#[test]
fn occ003_comparison_map_fails_closed_on_nonfinite_or_imported_structure() {
    let comparison_map = FiniteCandidateToCatalogComparisonMap::canonical_occ003();

    let missing_occ002 = FiniteCandidateToCatalogComparisonMap {
        occ002_finite_external_catalog_descriptor_closed: false,
        ..comparison_map
    };
    assert!(!missing_occ002.closes_occ003());

    let zero_candidate_bound = FiniteCandidateToCatalogComparisonMap {
        candidate_row_key_bound: 0,
        ..comparison_map
    };
    assert!(!zero_candidate_bound.closes_occ003());

    let edge_count_exceeds_bound = FiniteCandidateToCatalogComparisonMap {
        occupied_comparison_edge_count: comparison_map.comparison_edge_bound + 1,
        ..comparison_map
    };
    assert!(!edge_count_exceeds_bound.closes_occ003());

    let readout_exceeds_neighborhood = FiniteCandidateToCatalogComparisonMap {
        comparison_readout_boundary_size: comparison_map.local_comparison_neighborhood_size + 1,
        ..comparison_map
    };
    assert!(!readout_exceeds_neighborhood.closes_occ003());

    let missing_descriptor_support = FiniteCandidateToCatalogComparisonMap {
        descriptor_support_compatible: false,
        ..comparison_map
    };
    assert!(!missing_descriptor_support.closes_occ003());

    let physical_identification = FiniteCandidateToCatalogComparisonMap {
        physical_identification_map: true,
        ..comparison_map
    };
    assert!(!physical_identification.closes_occ003());

    let fit_only = FiniteCandidateToCatalogComparisonMap {
        fit_only_calibration: true,
        ..comparison_map
    };
    assert!(!fit_only.closes_occ003());

    let hidden_observed_catalog_recovery = FiniteCandidateToCatalogComparisonMap {
        observed_catalog_recovery_import: true,
        ..comparison_map
    };
    assert!(!hidden_observed_catalog_recovery.closes_occ003());

    let hidden_physical_standard_model = FiniteCandidateToCatalogComparisonMap {
        physical_standard_model_content_import: true,
        ..comparison_map
    };
    assert!(!hidden_physical_standard_model.closes_occ003());
}

#[test]
fn upstream_json_records_paper8_certificate_and_nonpromotion() {
    let root = project_root();
    let upstream = read(&root, "UPSTREAM-PAPERS.json");

    assert_contains(&upstream, PAPER1_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER2_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER3_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER4_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER5_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER6_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER7_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER8_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER8_FINAL_CERTIFICATE, "UPSTREAM-PAPERS.json");
    assert_contains(
        &upstream,
        "\"occ002_finite_external_catalog_descriptor_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"occ003_finite_candidate_to_catalog_comparison_map_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"observed_catalog_comparison_observables_theorem_closed\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"observed_particle_catalog_recovery_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"fit_only_calibration_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"unified_field_theory_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
}

#[test]
fn docs_keep_occ002_active_and_physical_claims_false() {
    let root = project_root();
    let theorem = read(
        &root,
        "docs/observed_catalog_comparison_observables_theorem.md",
    );
    let state = read(&root, "GPD/STATE.md");
    let readme = read(&root, "README.md");

    for artifact in [
        ("theorem doc", theorem.as_str()),
        ("state", state.as_str()),
        ("readme", readme.as_str()),
    ] {
        assert_contains(artifact.1, "OCC-001", artifact.0);
        assert_contains(artifact.1, "OCC-002", artifact.0);
        assert_contains(artifact.1, "OCC-003", artifact.0);
        assert_contains(artifact.1, "OCC-004", artifact.0);
        assert_contains(artifact.1, "observed particle catalog recovery", artifact.0);
        assert_contains(artifact.1, "physical Standard Model", artifact.0);
        assert_contains(artifact.1, "fit-only", artifact.0);
        assert_contains(artifact.1, "continuum quantum field theory", artifact.0);
        assert_contains(artifact.1, "unified", artifact.0);
    }
}

#[test]
fn repository_contains_no_python_artifacts() {
    let root = project_root();
    let mut python_artifacts = Vec::new();
    collect_python_artifacts(&root, &mut python_artifacts);

    assert!(
        python_artifacts.is_empty(),
        "Paper 9 scaffold must remain Rust-only; found Python artifacts: {python_artifacts:?}"
    );
}
