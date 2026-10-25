//! Signed technology a test provisions into the model.
//!
//! For tests whose subject requires a successful governed simulation. Tests
//! for missing, stale, revoked, or tampered authority must not call these
//! helpers. Keeping them opt-in preserves the production fail-closed default
//! while avoiding hand-built partial authority in unrelated simulation
//! fixtures.

use super::Model;
use crate::state::pdk_config::{PdkAdministrativeAuthority, PdkPublisherTrustStore};

impl Model {
    /// Provision one fully authenticated model-source and signed-PDK contract
    /// for tests whose subject requires a successful governed simulation.
    pub fn provision_test_project_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_technology_test_fixture(),
        );
    }

    /// The same contract, with `models` as the signed package's model library.
    pub fn provision_test_project_technology_with_models(&mut self, models: Vec<u8>) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_technology_test_fixture_with_models(models),
        );
    }

    /// The same contract, with the signed package's Verilog-A technology.
    pub fn provision_test_project_veriloga_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_veriloga_technology_test_fixture(),
        );
    }

    /// The same contract, with the signed package's symbol technology.
    pub fn provision_test_project_symbol_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_symbol_technology_test_fixture(),
        );
    }

    fn provision_test_project_technology_fixture(
        &mut self,
        (archive, trust, administrative_authority): (
            Vec<u8>,
            PdkPublisherTrustStore,
            PdkAdministrativeAuthority,
        ),
    ) {
        self.pdk_config.publisher_trust_store = trust.clone();
        self.pdk_config
            .technology_registry
            .install_archive_bytes(
                &archive,
                &trust,
                &administrative_authority,
                "Install deterministic test technology",
            )
            .expect("signed test technology installs");
        let package = self.pdk_config.technology_registry.validated_packages()[0].clone();

        let library_name = self
            .model_library_manager
            .load_library_bytes(
                "demo180-test.lib",
                b".model nmos_project_demo nmos level=1 vto=0.55".to_vec(),
                None,
            )
            .expect("authenticated retained test model imports");
        let library = self
            .model_library_manager
            .get_library(&library_name)
            .expect("imported test model remains in the catalog");
        let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
            .and_then(|binding| binding.with_signed_package_metadata(package.metadata()))
            .expect("model and signed package form one exact test binding");
        let authority = crate::state::ProjectTechnologyChangeAuthority::new(
            "simulation-test@rspice.invalid",
            "test:governed-simulation",
            "Provision deterministic signed simulation authority",
        )
        .expect("test authority is valid");
        let context = crate::state::ProjectTechnologyChangeContext::new(
            authority,
            uuid::Uuid::new_v4(),
            self.workspace.content.project.revision(),
            1,
            crate::product::ContentDigest::from_bytes([0x7a; 32]),
            1,
        )
        .expect("test checkpoint evidence is valid");
        self.workspace
            .content
            .project
            .attach_technology_audited(binding, context)
            .expect("test technology attachment commits with an audit receipt");
        self.technology_inputs()
            .validate_project_technology_contract()
            .expect("provisioned test technology resolves exactly");
    }
}
