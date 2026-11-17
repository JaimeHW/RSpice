//! PDK technology a test provisions into the model.
//!
//! For tests whose subject requires a successful governed simulation. Tests
//! for missing, stale, or tampered authority must not call these
//! helpers. Keeping them opt-in preserves the production fail-closed default
//! while avoiding hand-built partial authority in unrelated simulation
//! fixtures.

use super::Model;
use crate::state::pdk_config::PdkAdministrativeAuthority;

/// The project model library every contract binds unless a test names its
/// own: one model and no sections.
const PROJECT_MODELS: &[u8] = b".model nmos_project_demo nmos level=1 vto=0.55";

impl Model {
    /// Provision one fully authenticated model-source and PDK contract
    /// for tests whose subject requires a successful governed simulation.
    pub fn provision_test_project_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::pdk_technology_test_fixture(),
            PROJECT_MODELS.to_vec(),
        );
    }

    /// The same contract, with `models` as the PDK package's model library.
    pub fn provision_test_project_technology_with_models(&mut self, models: Vec<u8>) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::pdk_technology_test_fixture_with_models(models),
            PROJECT_MODELS.to_vec(),
        );
    }

    /// The same contract, with `models` as the project's model library, so
    /// the technology offers the sections `models` holds.
    pub fn provision_test_project_technology_with_project_models(&mut self, models: Vec<u8>) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::pdk_technology_test_fixture(),
            models,
        );
    }

    /// The same contract, with the PDK package's Verilog-A technology.
    pub fn provision_test_project_veriloga_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::pdk_veriloga_technology_test_fixture(),
            PROJECT_MODELS.to_vec(),
        );
    }

    /// The same contract, with the PDK package's symbol technology.
    pub fn provision_test_project_symbol_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::pdk_symbol_technology_test_fixture(),
            PROJECT_MODELS.to_vec(),
        );
    }

    fn provision_test_project_technology_fixture(
        &mut self,
        (archive, administrative_authority): (Vec<u8>, PdkAdministrativeAuthority),
        project_models: Vec<u8>,
    ) {
        self.pdk_config
            .technology_registry
            .install_archive_bytes(
                &archive,
                &administrative_authority,
                "Install deterministic test technology",
            )
            .expect("test PDK technology installs");
        let package = self.pdk_config.technology_registry.validated_packages()[0].clone();

        let library_name = self
            .model_library_manager
            .load_library_bytes("demo180-test.lib", project_models, None)
            .expect("authenticated retained test model imports");
        let library = self
            .model_library_manager
            .get_library(&library_name)
            .expect("imported test model remains in the catalog");
        let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
            .and_then(|binding| binding.with_pdk_package_metadata(package.metadata()))
            .expect("model and PDK package form one exact test binding");
        let authority = crate::state::ProjectTechnologyChangeAuthority::new(
            "simulation-test@rspice.invalid",
            "test:governed-simulation",
            "Provision deterministic PDK simulation authority",
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
