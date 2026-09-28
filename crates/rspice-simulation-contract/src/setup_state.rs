//! Portable setup restoration and compatibility state around the saved document.
//!
//! The document remains the only serialized authority. Legacy projections are
//! retained for migration and consumers that still require singleton drafts.

mod legacy_projection;
#[cfg(test)]
mod tests;

use crate::drafts::{AcSetup, DcSetup, TranSetup};
use crate::legacy_plan_migration::{NoiseSetup, default_global_run_set};
use crate::plan_catalog::{
    SimulationPlanCatalogError, SimulationPlanCloneOptions, SimulationPlanCloneOutcome,
    SimulationPlanImportDocument,
};
use crate::run_set::{ReferencePoint as ReferencePvtPoint, RunSetDimensionKind};
use rspice_app_types::product::SimulationPlanId;
use std::collections::HashSet;

/// Portable setup with the compatibility projections still used by legacy consumers.
#[derive(Debug, Clone, Default)]
pub struct SimulationSetup {
    document: crate::setup_document::SimulationSetupDocument,
    /// Enabled analysis indices.
    pub enabled: HashSet<usize>,
    /// Stable execution order. Enabled analyses absent from this vector are
    /// appended deterministically; disabled entries are ignored and removed
    /// from the persisted normalized plan.
    pub analysis_order: Vec<usize>,
    /// Transient sweep.
    pub tran: TranSetup,
    /// AC sweep.
    pub ac: AcSetup,
    /// DISTO secondary tone ratio f2/f1 (empty = single-tone HD).
    pub disto_f2_over_f1: String,
    /// DC transfer sweep.
    pub dc: DcSetup,
    /// Noise analysis.
    pub noise: NoiseSetup,
    /// DC operating point.
    pub op: crate::op_draft::OpDialogState,
    /// Pole-zero extraction.
    pub pz: crate::pz_draft::PzDialogState,
    /// Sensitivity.
    pub sens: crate::sens_draft::SensDialogState,
    /// Monte Carlo.
    pub mc: crate::mc_draft::McDialogState,
    /// Periodic steady state.
    pub pss: crate::pss_draft::PssDialogState,
    /// Loop stability.
    pub stb: crate::stb_draft::StbDialogState,
    /// Temperature sweep.
    pub temp: crate::temp_draft::TempDialogState,
    /// Harmonic balance.
    pub hb: crate::hb_draft::HbDialogState,
    /// S-parameters.
    pub sp: crate::sp_draft::SpDialogState,
    /// Periodic AC.
    pub pac: crate::pac_draft::PacDialogState,
    /// Periodic noise.
    pub pnoise: crate::pnoise_draft::PnoiseDialogState,
    /// Periodic transfer.
    pub pxf: crate::pxf_draft::PxfDialogState,
    /// Periodic stability.
    pub pstb: crate::pstb_draft::PstbDialogState,
    /// Transfer function.
    pub xf: crate::xf_draft::XfDialogState,
    /// Process corners.
    pub corner: crate::corner_draft::CornerDialogState,
    /// Envelope transient.
    pub envelope: crate::envelope_draft::EnvelopeDialogState,
    /// Fourier.
    pub fourier: crate::fourier_draft::FourierDialogState,
    /// Optimization.
    pub optimization: crate::optimization_draft::OptimizationDialogState,
    /// Safe operating area.
    pub soa: crate::soa_draft::SoaDialogState,
    /// Analyses listed in the run-set card beyond the always-listed core —
    /// exotics stay listed (dimmed) when unticked, until removed.
    pub listed: HashSet<usize>,
    /// One-shot evidence retained when a legacy run-set declaration is superseded.
    pub legacy_run_set_notes: Vec<String>,
}

impl std::ops::Deref for SimulationSetup {
    type Target = crate::setup_document::SimulationSetupDocument;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl std::ops::DerefMut for SimulationSetup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}

impl serde::Serialize for SimulationSetup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.document, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for SimulationSetup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let read = <crate::setup_document::legacy_read::LegacySimulationSetupRead as serde::Deserialize>::deserialize(deserializer)?;
        let crate::setup_document::legacy_read::LegacySimulationSetupRead {
            reference_pvt,
            run_set,
            model_bindings,
            save_policy,
            active_plan_name,
            active_plan_lineage,
            inactive_plans,
            analysis_plan,
            options,
            enabled,
            analysis_order,
            tran,
            ac,
            disto_f2_over_f1,
            dc,
            noise,
            op,
            pz,
            sens,
            mc,
            pss,
            stb,
            temp,
            hb,
            sp,
            pac,
            pnoise,
            pxf,
            pstb,
            xf,
            corner,
            envelope,
            fourier,
            optimization,
            soa,
            listed,
        } = read;
        Ok(Self {
            document: crate::setup_document::SimulationSetupDocument {
                reference_pvt,
                run_set,
                model_bindings,
                save_policy,
                active_plan_name,
                active_plan_lineage,
                inactive_plans,
                analysis_plan,
                options,
            },
            enabled,
            analysis_order,
            tran,
            ac,
            disto_f2_over_f1,
            dc,
            noise,
            op,
            pz,
            sens,
            mc,
            pss,
            stb,
            temp,
            hb,
            sp,
            pac,
            pnoise,
            pxf,
            pstb,
            xf,
            corner,
            envelope,
            fourier,
            optimization,
            soa,
            listed,
            legacy_run_set_notes: Vec::new(),
        })
    }
}

impl SimulationSetup {
    /// Fresh setup with the conventional default run set — a transient —
    /// so a new project's Run button works out of the box (the engine no
    /// longer falls back to the selected row on an empty set).
    pub fn new() -> Self {
        let mut setup = Self::default();
        setup.document.analysis_plan = Some(crate::plan_model::SimulationPlan::new());
        setup.document.run_set = default_global_run_set();
        setup
            .set_reference_pvt(rspice_app_types::product::ProcessCorner::TT, 27.0)
            .expect("the built-in reference PVT point is valid");
        setup
            .enabled
            .insert(crate::analysis_kind::AnalysisKind::Transient.legacy_index());
        setup
            .analysis_order
            .push(crate::analysis_kind::AnalysisKind::Transient.legacy_index());
        setup
    }

    /// Rebuild transient editing state after a persisted plan is restored.
    pub fn prepare_after_restore(&mut self) {
        if let Some(plan) = &mut self.analysis_plan {
            plan.prepare_after_restore();
        }
        self.prepare_plan_catalog_after_restore();
        self.op.initialized = true;
        self.pz.initialized = true;
        self.sens.initialized = true;
        self.mc.initialized = true;
        self.pss.initialized = true;
        self.stb.initialized = true;
        self.temp.initialized = true;
        self.hb.initialized = true;
        self.sp.initialized = true;
        self.pac.initialized = true;
        self.pnoise.initialized = true;
        self.pxf.initialized = true;
        self.pstb.initialized = true;
        self.xf.prepare_after_restore();
        self.corner.initialized = true;
        self.envelope.initialized = true;
        self.fourier.initialized = true;
        self.optimization.initialized = true;
        self.soa.initialized = true;
        self.refresh_legacy_analysis_projections();
    }

    /// Select the nominal/reference PVT point consumed by subsequent runs.
    pub fn set_reference_pvt(
        &mut self,
        process: rspice_app_types::product::ProcessCorner,
        temperature_celsius: f64,
    ) -> Result<(), String> {
        if !temperature_celsius.is_finite() {
            return Err("Reference temperature must be finite".to_owned());
        }
        if temperature_celsius <= -273.15 {
            return Err("Reference temperature must be above absolute zero".to_owned());
        }

        self.reference_pvt = ReferencePvtPoint {
            process,
            temperature_celsius,
        };
        self.options.temp = temperature_celsius;
        self.op.ensure_initialized();
        self.op.temperature = temperature_celsius.to_string();
        Ok(())
    }

    /// Every temperature this run set asks the engine for, in °C.
    ///
    /// The same rule the corner projection uses: a declared temperature axis
    /// is the request, and without one the reference point is the request —
    /// exactly once, because a plan with no axis runs at one temperature. It
    /// is stated here so a surface asking "is this corner qualified for what
    /// we are about to run" reads the run set rather than guessing from the
    /// reference point alone.
    #[must_use]
    pub fn requested_temperatures_celsius(&self) -> Vec<f64> {
        match self
            .run_set
            .enabled_dimension_of(RunSetDimensionKind::Temperature)
        {
            Some(dimension) => dimension.canonical_values(),
            None => vec![self.reference_pvt.temperature_celsius],
        }
    }

    /// Commit globally validated options while keeping the workbench reference
    /// point and OP editor aligned with the temperature the solver will use.
    pub fn commit_options(&mut self, options: &crate::options::SimulationOptions) {
        self.options = options.clone();
        self.reference_pvt.temperature_celsius = options.temp;
        self.op.ensure_initialized();
        self.op.temperature = options.temp.to_string();
    }

    pub fn create_plan(
        &mut self,
        name: impl Into<String>,
    ) -> Result<SimulationPlanId, SimulationPlanCatalogError> {
        let id = self.document.create_plan(name)?;
        self.refresh_legacy_analysis_projections();
        Ok(id)
    }

    pub fn import_plan(
        &mut self,
        document: SimulationPlanImportDocument,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        let outcome = self.document.import_plan(document)?;
        self.refresh_legacy_analysis_projections();
        Ok(outcome)
    }

    pub fn clone_active_plan(
        &mut self,
        new_name: impl Into<String>,
        contents: SimulationPlanCloneOptions,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        let outcome = self.document.clone_active_plan(new_name, contents)?;
        self.refresh_legacy_analysis_projections();
        Ok(outcome)
    }

    pub fn activate_plan(
        &mut self,
        id: SimulationPlanId,
    ) -> Result<(), SimulationPlanCatalogError> {
        let previous = self.document.analysis_plan.as_ref().map(|plan| plan.id());
        self.document.activate_plan(id)?;
        if previous != Some(id) {
            self.refresh_legacy_analysis_projections();
        }
        Ok(())
    }
}
