use super::*;

//=============================================================================
// Memory
//=============================================================================

/// RAM model
#[derive(Debug, Default)]
pub struct DigitalRam;

impl CodeModel for DigitalRam {
    fn name(&self) -> &str {
        "d_ram"
    }
    fn description(&self) -> &str {
        "Random access memory"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("data_in", PortType::Digital),
                PortSpec::vector_output("data_out", PortType::Digital),
                PortSpec::vector_input("address", PortType::Digital),
                PortSpec::input("write_en", PortType::Digital),
                PortSpec::input("select", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("read_delay", 1e-9).with_min(0.0),
                ParamSpec::real("write_delay", 1e-9).with_min(0.0),
                ParamSpec::integer("address_width", 8),
                ParamSpec::integer("data_width", 8),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}

/// ROM model
#[derive(Debug, Default)]
pub struct DigitalRom;

impl CodeModel for DigitalRom {
    fn name(&self) -> &str {
        "d_rom"
    }
    fn description(&self) -> &str {
        "Read-only memory"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("address", PortType::Digital),
                PortSpec::input("select", PortType::Digital),
                PortSpec::vector_output("data_out", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalRam.parameters()
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}
