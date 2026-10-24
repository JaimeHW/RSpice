//=============================================================================
// Types
//=============================================================================

/// VDMOS device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdmosType {
    /// N-channel VDMOS (most common for power switching)
    NVdmos,
    /// P-channel VDMOS
    PVdmos,
}

/// Operating region of VDMOS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdmosRegion {
    /// Gate below threshold, no channel conduction
    Cutoff,
    /// Linear/ohmic region (Vds < Vgs - Vth)
    Triode,
    /// Normal saturation region
    Saturation,
    /// Quasi-saturation: drift region limiting current
    QuasiSaturation,
    /// Body diode conducting (reverse Vds)
    BodyDiode,
}
