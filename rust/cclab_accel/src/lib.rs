pub const PAPER1_FROZEN_COMMIT: &str = "3a9637c65f783ca35e77118f83560290f42f3085";
pub const PAPER2_FROZEN_COMMIT: &str = "053842ef5e1a50282df9d884266e87428ee07f60";
pub const PAPER3_FROZEN_COMMIT: &str = "6067360758108f799fa604855f5513545019492e";
pub const PAPER4_FROZEN_COMMIT: &str = "5a1ac95700786b697a0f25ddecb393fdeaaa166e";
pub const PAPER5_FROZEN_COMMIT: &str = "8db1a334b0c0ca934ccd3628add72c6e3f1ebfcb";
pub const PAPER6_FROZEN_COMMIT: &str = "20df751a0ceb2b4eb33a80dd15dd2795a1ea529a";
pub const PAPER7_FROZEN_COMMIT: &str = "4f52d9980f62977016ef5ee5da9e88a32dce70e5";
pub const PAPER8_FROZEN_COMMIT: &str = "d3c58356cdbe89d9a8b7a79784c7b6eaf4023b33";
pub const PAPER8_FINAL_CERTIFICATE: &str =
    "paper8_smc008_final_conditional_certificate_closes_standard_model_candidate_observables_theorem";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper9UpstreamBinding {
    pub paper1_commit: &'static str,
    pub paper2_commit: &'static str,
    pub paper3_commit: &'static str,
    pub paper4_commit: &'static str,
    pub paper5_commit: &'static str,
    pub paper6_commit: &'static str,
    pub paper7_commit: &'static str,
    pub paper8_commit: &'static str,
    pub paper1_internal_conditional_closed: bool,
    pub paper2_geometry_closed: bool,
    pub paper3_curvature_closed: bool,
    pub paper4_dynamics_closed: bool,
    pub paper5_quantum_compatible_local_dynamics_closed: bool,
    pub paper6_matter_gauge_observables_closed: bool,
    pub paper7_particle_excitation_observables_closed: bool,
    pub paper8_standard_model_candidate_observables_closed: bool,
    pub paper8_final_certificate_consumed: bool,
    pub may_modify_upstreams: bool,
    pub physical_nature_claim: bool,
    pub continuum_qft_claim: bool,
    pub physical_matter_fields_claim: bool,
    pub physical_gauge_fields_claim: bool,
    pub physical_particle_excitation_claim: bool,
    pub observed_particle_catalog_recovery_claim: bool,
    pub physical_standard_model_claim: bool,
    pub observed_catalog_comparison_observable_recovery_claim: bool,
    pub physical_quantum_dynamics_claim: bool,
    pub simulation_only_promotion: bool,
    pub fit_only_calibration_claim: bool,
    pub unified_field_theory_claim: bool,
}

impl Paper9UpstreamBinding {
    pub fn canonical_occ001() -> Self {
        Self {
            paper1_commit: PAPER1_FROZEN_COMMIT,
            paper2_commit: PAPER2_FROZEN_COMMIT,
            paper3_commit: PAPER3_FROZEN_COMMIT,
            paper4_commit: PAPER4_FROZEN_COMMIT,
            paper5_commit: PAPER5_FROZEN_COMMIT,
            paper6_commit: PAPER6_FROZEN_COMMIT,
            paper7_commit: PAPER7_FROZEN_COMMIT,
            paper8_commit: PAPER8_FROZEN_COMMIT,
            paper1_internal_conditional_closed: true,
            paper2_geometry_closed: true,
            paper3_curvature_closed: true,
            paper4_dynamics_closed: true,
            paper5_quantum_compatible_local_dynamics_closed: true,
            paper6_matter_gauge_observables_closed: true,
            paper7_particle_excitation_observables_closed: true,
            paper8_standard_model_candidate_observables_closed: true,
            paper8_final_certificate_consumed: true,
            may_modify_upstreams: false,
            physical_nature_claim: false,
            continuum_qft_claim: false,
            physical_matter_fields_claim: false,
            physical_gauge_fields_claim: false,
            physical_particle_excitation_claim: false,
            observed_particle_catalog_recovery_claim: false,
            physical_standard_model_claim: false,
            observed_catalog_comparison_observable_recovery_claim: false,
            physical_quantum_dynamics_claim: false,
            simulation_only_promotion: false,
            fit_only_calibration_claim: false,
            unified_field_theory_claim: false,
        }
    }

    pub fn closes_occ001(&self) -> bool {
        self.paper1_commit == PAPER1_FROZEN_COMMIT
            && self.paper2_commit == PAPER2_FROZEN_COMMIT
            && self.paper3_commit == PAPER3_FROZEN_COMMIT
            && self.paper4_commit == PAPER4_FROZEN_COMMIT
            && self.paper5_commit == PAPER5_FROZEN_COMMIT
            && self.paper6_commit == PAPER6_FROZEN_COMMIT
            && self.paper7_commit == PAPER7_FROZEN_COMMIT
            && self.paper8_commit == PAPER8_FROZEN_COMMIT
            && self.paper1_internal_conditional_closed
            && self.paper2_geometry_closed
            && self.paper3_curvature_closed
            && self.paper4_dynamics_closed
            && self.paper5_quantum_compatible_local_dynamics_closed
            && self.paper6_matter_gauge_observables_closed
            && self.paper7_particle_excitation_observables_closed
            && self.paper8_standard_model_candidate_observables_closed
            && self.paper8_final_certificate_consumed
            && !self.may_modify_upstreams
            && !self.physical_nature_claim
            && !self.continuum_qft_claim
            && !self.physical_matter_fields_claim
            && !self.physical_gauge_fields_claim
            && !self.physical_particle_excitation_claim
            && !self.observed_particle_catalog_recovery_claim
            && !self.physical_standard_model_claim
            && !self.observed_catalog_comparison_observable_recovery_claim
            && !self.physical_quantum_dynamics_claim
            && !self.simulation_only_promotion
            && !self.fit_only_calibration_claim
            && !self.unified_field_theory_claim
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper9SkeletonCertificate {
    pub occ001_upstream_binding_closed: bool,
    pub occ002_finite_external_catalog_descriptor_closed: bool,
    pub occ003_finite_candidate_to_catalog_comparison_map_closed: bool,
    pub occ004_standard_model_candidate_compatibility_closed: bool,
    pub occ005_comparison_coarse_graining_stability_closed: bool,
    pub occ006_paper8_regime_consistency_closed: bool,
    pub occ007_no_hidden_observed_recovery_fit_audit_closed: bool,
    pub occ008_final_conditional_certificate_closed: bool,
    pub paper9_theorem_closed: bool,
    pub physical_nature_claim: bool,
    pub observed_particle_catalog_recovery_claim: bool,
    pub physical_standard_model_claim: bool,
    pub physical_particle_excitation_claim: bool,
    pub physical_matter_fields_claim: bool,
    pub physical_gauge_fields_claim: bool,
    pub physical_quantum_dynamics_claim: bool,
    pub continuum_qft_claim: bool,
    pub simulation_only_promotion: bool,
    pub fit_only_calibration_claim: bool,
    pub unified_field_theory_claim: bool,
}

impl Paper9SkeletonCertificate {
    pub fn initial_occ001_only() -> Self {
        let binding = Paper9UpstreamBinding::canonical_occ001();
        Self {
            occ001_upstream_binding_closed: binding.closes_occ001(),
            occ002_finite_external_catalog_descriptor_closed: false,
            occ003_finite_candidate_to_catalog_comparison_map_closed: false,
            occ004_standard_model_candidate_compatibility_closed: false,
            occ005_comparison_coarse_graining_stability_closed: false,
            occ006_paper8_regime_consistency_closed: false,
            occ007_no_hidden_observed_recovery_fit_audit_closed: false,
            occ008_final_conditional_certificate_closed: false,
            paper9_theorem_closed: false,
            physical_nature_claim: false,
            observed_particle_catalog_recovery_claim: false,
            physical_standard_model_claim: false,
            physical_particle_excitation_claim: false,
            physical_matter_fields_claim: false,
            physical_gauge_fields_claim: false,
            physical_quantum_dynamics_claim: false,
            continuum_qft_claim: false,
            simulation_only_promotion: false,
            fit_only_calibration_claim: false,
            unified_field_theory_claim: false,
        }
    }

    pub fn closes_paper9_theorem(&self) -> bool {
        self.occ001_upstream_binding_closed
            && self.occ002_finite_external_catalog_descriptor_closed
            && self.occ003_finite_candidate_to_catalog_comparison_map_closed
            && self.occ004_standard_model_candidate_compatibility_closed
            && self.occ005_comparison_coarse_graining_stability_closed
            && self.occ006_paper8_regime_consistency_closed
            && self.occ007_no_hidden_observed_recovery_fit_audit_closed
            && self.occ008_final_conditional_certificate_closed
            && self.paper9_theorem_closed
            && !self.physical_nature_claim
            && !self.observed_particle_catalog_recovery_claim
            && !self.physical_standard_model_claim
            && !self.physical_particle_excitation_claim
            && !self.physical_matter_fields_claim
            && !self.physical_gauge_fields_claim
            && !self.physical_quantum_dynamics_claim
            && !self.continuum_qft_claim
            && !self.simulation_only_promotion
            && !self.fit_only_calibration_claim
            && !self.unified_field_theory_claim
    }
}

pub fn paper9_skeleton_marker() -> &'static str {
    "paper9-observed-catalog-comparison-observables-occ001-nonpromoting-skeleton"
}
