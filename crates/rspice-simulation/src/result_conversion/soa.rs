//! Canonical safety quantities retained from the engine report.

use rspice_results::soa_evidence::SoaParameterEvidence;

pub(super) fn retain_soa_parameter(
    parameter: rspice_results::safety::SoAParameter,
) -> SoaParameterEvidence {
    match parameter {
        rspice_results::safety::SoAParameter::Vgs => SoaParameterEvidence::GateSourceVoltage,
        rspice_results::safety::SoAParameter::Vds => SoaParameterEvidence::DrainSourceVoltage,
        rspice_results::safety::SoAParameter::Vgd => SoaParameterEvidence::GateDrainVoltage,
        rspice_results::safety::SoAParameter::Vbe => SoaParameterEvidence::BaseEmitterVoltage,
        rspice_results::safety::SoAParameter::Vce => SoaParameterEvidence::CollectorEmitterVoltage,
        rspice_results::safety::SoAParameter::Vbc => SoaParameterEvidence::BaseCollectorVoltage,
        rspice_results::safety::SoAParameter::Id => SoaParameterEvidence::DrainCurrent,
        rspice_results::safety::SoAParameter::Ic => SoaParameterEvidence::CollectorCurrent,
        rspice_results::safety::SoAParameter::Vcsub => {
            SoaParameterEvidence::CollectorSubstrateVoltage
        }
        rspice_results::safety::SoAParameter::VcsubPositive => {
            SoaParameterEvidence::CollectorSubstrateVoltagePositive
        }
        rspice_results::safety::SoAParameter::VcsubNegative => {
            SoaParameterEvidence::CollectorSubstrateVoltageNegative
        }
        rspice_results::safety::SoAParameter::Vbsub => SoaParameterEvidence::BaseSubstrateVoltage,
        rspice_results::safety::SoAParameter::VbsubPositive => {
            SoaParameterEvidence::BaseSubstrateVoltagePositive
        }
        rspice_results::safety::SoAParameter::VbsubNegative => {
            SoaParameterEvidence::BaseSubstrateVoltageNegative
        }
        rspice_results::safety::SoAParameter::Vesub => {
            SoaParameterEvidence::EmitterSubstrateVoltage
        }
        rspice_results::safety::SoAParameter::VesubPositive => {
            SoaParameterEvidence::EmitterSubstrateVoltagePositive
        }
        rspice_results::safety::SoAParameter::VesubNegative => {
            SoaParameterEvidence::EmitterSubstrateVoltageNegative
        }
        rspice_results::safety::SoAParameter::Isub => SoaParameterEvidence::SubstrateCurrent,
        rspice_results::safety::SoAParameter::IsubPositive => {
            SoaParameterEvidence::SubstrateCurrentPositive
        }
        rspice_results::safety::SoAParameter::IsubNegative => {
            SoaParameterEvidence::SubstrateCurrentNegative
        }
        rspice_results::safety::SoAParameter::Vak => SoaParameterEvidence::AnodeCathodeVoltage,
        rspice_results::safety::SoAParameter::VakPositive => {
            SoaParameterEvidence::AnodeCathodeVoltagePositive
        }
        rspice_results::safety::SoAParameter::VakNegative => {
            SoaParameterEvidence::AnodeCathodeVoltageNegative
        }
        rspice_results::safety::SoAParameter::Ia => SoaParameterEvidence::AnodeCurrent,
        rspice_results::safety::SoAParameter::IaPositive => {
            SoaParameterEvidence::AnodeCurrentPositive
        }
        rspice_results::safety::SoAParameter::IaNegative => {
            SoaParameterEvidence::AnodeCurrentNegative
        }

        rspice_results::safety::SoAParameter::Vbs => SoaParameterEvidence::BodySourceVoltage,
        rspice_results::safety::SoAParameter::VbsPositive => {
            SoaParameterEvidence::BodySourceVoltagePositive
        }
        rspice_results::safety::SoAParameter::VbsNegative => {
            SoaParameterEvidence::BodySourceVoltageNegative
        }
        rspice_results::safety::SoAParameter::Vbd => SoaParameterEvidence::BodyDrainVoltage,
        rspice_results::safety::SoAParameter::VbdPositive => {
            SoaParameterEvidence::BodyDrainVoltagePositive
        }
        rspice_results::safety::SoAParameter::VbdNegative => {
            SoaParameterEvidence::BodyDrainVoltageNegative
        }
        rspice_results::safety::SoAParameter::Vgb => SoaParameterEvidence::GateBodyVoltage,
        rspice_results::safety::SoAParameter::VgbPositive => {
            SoaParameterEvidence::GateBodyVoltagePositive
        }
        rspice_results::safety::SoAParameter::VgbNegative => {
            SoaParameterEvidence::GateBodyVoltageNegative
        }
        rspice_results::safety::SoAParameter::Ibulk => SoaParameterEvidence::BulkCurrent,
        rspice_results::safety::SoAParameter::IbulkPositive => {
            SoaParameterEvidence::BulkCurrentPositive
        }
        rspice_results::safety::SoAParameter::IbulkNegative => {
            SoaParameterEvidence::BulkCurrentNegative
        }
        rspice_results::safety::SoAParameter::Ves => SoaParameterEvidence::BackgateSourceVoltage,
        rspice_results::safety::SoAParameter::VesPositive => {
            SoaParameterEvidence::BackgateSourceVoltagePositive
        }
        rspice_results::safety::SoAParameter::VesNegative => {
            SoaParameterEvidence::BackgateSourceVoltageNegative
        }
        rspice_results::safety::SoAParameter::Ved => SoaParameterEvidence::BackgateDrainVoltage,
        rspice_results::safety::SoAParameter::VedPositive => {
            SoaParameterEvidence::BackgateDrainVoltagePositive
        }
        rspice_results::safety::SoAParameter::VedNegative => {
            SoaParameterEvidence::BackgateDrainVoltageNegative
        }
        rspice_results::safety::SoAParameter::Vge => SoaParameterEvidence::GateBackgateVoltage,
        rspice_results::safety::SoAParameter::VgePositive => {
            SoaParameterEvidence::GateBackgateVoltagePositive
        }
        rspice_results::safety::SoAParameter::VgeNegative => {
            SoaParameterEvidence::GateBackgateVoltageNegative
        }
        rspice_results::safety::SoAParameter::Ibackgate => SoaParameterEvidence::BackgateCurrent,
        rspice_results::safety::SoAParameter::IbackgatePositive => {
            SoaParameterEvidence::BackgateCurrentPositive
        }
        rspice_results::safety::SoAParameter::IbackgateNegative => {
            SoaParameterEvidence::BackgateCurrentNegative
        }
        rspice_results::safety::SoAParameter::VbodyBackgate => {
            SoaParameterEvidence::BodyBackgateVoltage
        }
        rspice_results::safety::SoAParameter::VbodyBackgatePositive => {
            SoaParameterEvidence::BodyBackgateVoltagePositive
        }
        rspice_results::safety::SoAParameter::VbodyBackgateNegative => {
            SoaParameterEvidence::BodyBackgateVoltageNegative
        }

        rspice_results::safety::SoAParameter::Ig => SoaParameterEvidence::GateCurrent,
        rspice_results::safety::SoAParameter::IgPositive => {
            SoaParameterEvidence::GateCurrentPositive
        }
        rspice_results::safety::SoAParameter::IgNegative => {
            SoaParameterEvidence::GateCurrentNegative
        }
        rspice_results::safety::SoAParameter::Is => SoaParameterEvidence::SourceCurrent,
        rspice_results::safety::SoAParameter::IsPositive => {
            SoaParameterEvidence::SourceCurrentPositive
        }
        rspice_results::safety::SoAParameter::IsNegative => {
            SoaParameterEvidence::SourceCurrentNegative
        }
        rspice_results::safety::SoAParameter::Ib => SoaParameterEvidence::BaseCurrent,
        rspice_results::safety::SoAParameter::IbPositive => {
            SoaParameterEvidence::BaseCurrentPositive
        }
        rspice_results::safety::SoAParameter::IbNegative => {
            SoaParameterEvidence::BaseCurrentNegative
        }
        rspice_results::safety::SoAParameter::Ie => SoaParameterEvidence::EmitterCurrent,
        rspice_results::safety::SoAParameter::IePositive => {
            SoaParameterEvidence::EmitterCurrentPositive
        }
        rspice_results::safety::SoAParameter::IeNegative => {
            SoaParameterEvidence::EmitterCurrentNegative
        }

        rspice_results::safety::SoAParameter::Pdiss => SoaParameterEvidence::PowerDissipation,
        rspice_results::safety::SoAParameter::Temp => SoaParameterEvidence::Temperature,
        rspice_results::safety::SoAParameter::VgsPositive => {
            SoaParameterEvidence::GateSourceVoltagePositive
        }
        rspice_results::safety::SoAParameter::VgsNegative => {
            SoaParameterEvidence::GateSourceVoltageNegative
        }
        rspice_results::safety::SoAParameter::VdsPositive => {
            SoaParameterEvidence::DrainSourceVoltagePositive
        }
        rspice_results::safety::SoAParameter::VdsNegative => {
            SoaParameterEvidence::DrainSourceVoltageNegative
        }
        rspice_results::safety::SoAParameter::VgdPositive => {
            SoaParameterEvidence::GateDrainVoltagePositive
        }
        rspice_results::safety::SoAParameter::VgdNegative => {
            SoaParameterEvidence::GateDrainVoltageNegative
        }
        rspice_results::safety::SoAParameter::VbePositive => {
            SoaParameterEvidence::BaseEmitterVoltagePositive
        }
        rspice_results::safety::SoAParameter::VbeNegative => {
            SoaParameterEvidence::BaseEmitterVoltageNegative
        }
        rspice_results::safety::SoAParameter::VcePositive => {
            SoaParameterEvidence::CollectorEmitterVoltagePositive
        }
        rspice_results::safety::SoAParameter::VceNegative => {
            SoaParameterEvidence::CollectorEmitterVoltageNegative
        }
        rspice_results::safety::SoAParameter::VbcPositive => {
            SoaParameterEvidence::BaseCollectorVoltagePositive
        }
        rspice_results::safety::SoAParameter::VbcNegative => {
            SoaParameterEvidence::BaseCollectorVoltageNegative
        }
        rspice_results::safety::SoAParameter::IdPositive => {
            SoaParameterEvidence::DrainCurrentPositive
        }
        rspice_results::safety::SoAParameter::IdNegative => {
            SoaParameterEvidence::DrainCurrentNegative
        }
        rspice_results::safety::SoAParameter::IcPositive => {
            SoaParameterEvidence::CollectorCurrentPositive
        }
        rspice_results::safety::SoAParameter::IcNegative => {
            SoaParameterEvidence::CollectorCurrentNegative
        }
    }
}
