use cclab_accel::{
    paper9_skeleton_marker, Paper9SkeletonCertificate, Paper9UpstreamBinding, PAPER1_FROZEN_COMMIT,
    PAPER2_FROZEN_COMMIT, PAPER3_FROZEN_COMMIT, PAPER4_FROZEN_COMMIT, PAPER5_FROZEN_COMMIT,
    PAPER6_FROZEN_COMMIT, PAPER7_FROZEN_COMMIT, PAPER8_FINAL_CERTIFICATE, PAPER8_FROZEN_COMMIT,
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
