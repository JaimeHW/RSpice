//! Property sheet for the MOSFET.

use super::*;

/// Which substrate the sheet is being built for. SOI instances carry body
/// and self-heating parameters that no bulk builder reads.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Bulk {
    Bulk,
    Soi,
}

impl PropertyRegistry {
    /// Register MOSFET (NMOS/PMOS) with all Spectre-compatible parameters
    pub(super) fn register_mosfet(&mut self) {
        let nmos = self.create_mosfet_sheet("M1", "nmos", Bulk::Bulk);
        self.sheets.insert(ComponentType::Nmos, nmos);

        let pmos = self.create_mosfet_sheet("M1", "pmos", Bulk::Bulk);
        self.sheets.insert(ComponentType::Pmos, pmos);

        // SOI devices run the bulk instance set plus the body and
        // self-heating terms only the B3SOI builder reads; the rest of the
        // SOI physics (SOIMOD, body tie) rides on the model card.
        let nmos_soi = self.create_mosfet_sheet("M1", "nmossoi", Bulk::Soi);
        self.sheets.insert(ComponentType::NmosSoi, nmos_soi);
        let pmos_soi = self.create_mosfet_sheet("M1", "pmossoi", Bulk::Soi);
        self.sheets.insert(ComponentType::PmosSoi, pmos_soi);

        self.register_vdmos();
    }

    /// Register the vertical power DMOS pair. The device physics (body
    /// diode, drift resistance, thermal network) live on the VDMOS model
    /// card, so the instance sheet stays lean.
    fn register_vdmos(&mut self) {
        for (kind, default_model) in [
            (ComponentType::NVdmos, "nvdmos"),
            (ComponentType::PVdmos, "pvdmos"),
        ] {
            let mut sheet = PropertySheet::new();

            sheet.add(
                PropertyDefinition::new("name")
                    .with_display_name("Instance Name")
                    .with_description("Unique identifier for this power MOSFET instance")
                    .with_type(PropertyType::String)
                    .with_default(PropertyValue::string("M1"))
                    .with_order(0)
                    .with_category("Instance")
                    .required(),
            );
            sheet.add(
                PropertyDefinition::new("model")
                    .with_display_name("Model")
                    .with_description("VDMOS model name from device library")
                    .with_type(PropertyType::String)
                    .with_default(PropertyValue::string(default_model))
                    .with_order(10)
                    .with_category("Model"),
            );
            sheet.add(
                PropertyDefinition::new("w")
                    .with_display_name("Width Scale")
                    .with_description("Channel width scale factor")
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression("1"))
                    .with_order(20)
                    .with_category("Geometry"),
            );
            sheet.add(
                PropertyDefinition::new("l")
                    .with_display_name("Length Scale")
                    .with_description("Channel length scale factor")
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression("1u"))
                    .with_order(21)
                    .with_category("Geometry"),
            );
            sheet.add(
                PropertyDefinition::new("m")
                    .with_display_name("Multiplier")
                    .with_description("Number of parallel devices")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(1.0))
                    .with_range(1.0, 10000.0)
                    .with_order(22)
                    .with_category("Geometry"),
            );
            // Body-diode diffusion geometry (order 23-26). The VDMOS builder
            // reads all four for the source and drain junction capacitance.
            for (name, display, description, unit, order) in [
                (
                    "as",
                    "Source Area",
                    "Source diffusion area for junction capacitance",
                    "m²",
                    23,
                ),
                (
                    "ad",
                    "Drain Area",
                    "Drain diffusion area for junction capacitance",
                    "m²",
                    24,
                ),
                (
                    "ps",
                    "Source Perimeter",
                    "Source diffusion perimeter for sidewall capacitance",
                    "m",
                    25,
                ),
                (
                    "pd",
                    "Drain Perimeter",
                    "Drain diffusion perimeter for sidewall capacitance",
                    "m",
                    26,
                ),
            ] {
                sheet.add(
                    PropertyDefinition::new(name)
                        .with_display_name(display)
                        .with_description(description)
                        .with_type(PropertyType::Expression)
                        .with_default(PropertyValue::expression("0"))
                        .with_unit(unit)
                        .with_order(order)
                        .with_category("Parasitics"),
                );
            }
            sheet.add(
                PropertyDefinition::new("temp")
                    .with_display_name("Temperature")
                    .with_description(
                        "Absolute device temperature; overrides the circuit temperature entirely",
                    )
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression(""))
                    .with_unit("°C")
                    .with_order(30)
                    .with_category("Temperature"),
            );
            sheet.add(
                PropertyDefinition::new("dtemp")
                    .with_display_name("Temp Rise")
                    .with_description("Instance temperature rise above ambient")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(0.0))
                    .with_unit("°C")
                    .with_order(31)
                    .with_category("Temperature"),
            );
            // No OFF switch: the VDMOS builder reads W/L, M, the diffusion
            // geometry, and TEMP/DTEMP only.

            self.sheets.insert(kind, sheet);
        }
    }

    /// Create a MOSFET property sheet with commercial-grade parameters
    fn create_mosfet_sheet(
        &self,
        default_name: &str,
        default_model: &str,
        bulk: Bulk,
    ) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this MOSFET instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // Model Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("MOSFET model name from PDK library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // =========================================================================
        // Geometry Category (order 20-39)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Channel width (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(20)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Channel length (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("180n"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(21)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices (total W = m × nf × w)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(22)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("nf")
                .with_display_name("# Fingers")
                .with_description("Number of gate fingers")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 1000.0)
                .with_order(23)
                .with_category("Geometry"),
        );
        // BSIM4 source/drain end-contact selectors (order 24-26). Integer
        // codes from the BSIM4 manual, validated by the builder against the
        // allowed set; only the native BSIM4 path reads them.
        sheet.add(
            PropertyDefinition::new("geomod")
                .with_display_name("Geometry Mode")
                .with_description(
                    "BSIM4 GEOMOD source/drain end-contact selector (0-10); overrides the model card",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 10.0)
                .with_order(24)
                .with_category("Geometry")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("rgeomod")
                .with_display_name("Contact Mode")
                .with_description(
                    "BSIM4 RGEOMOD source/drain contact-resistance selector (0-8); overrides the model card",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 8.0)
                .with_order(25)
                .with_category("Geometry")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("min")
                .with_display_name("Minimize End")
                .with_description(
                    "BSIM4 MIN selector: 0 minimizes the drain diffusion, 1 the source",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1.0)
                .with_order(26)
                .with_category("Geometry")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-59)
        // Source/Drain areas and perimeters for junction capacitance
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("as")
                .with_display_name("Source Area")
                .with_description("Source diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ad")
                .with_display_name("Drain Area")
                .with_description("Drain diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ps")
                .with_display_name("Source Perimeter")
                .with_description("Source diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(42)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("pd")
                .with_display_name("Drain Perimeter")
                .with_description("Drain diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(43)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrd")
                .with_display_name("Drain Squares")
                .with_description("Number of squares for drain series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(44)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrs")
                .with_display_name("Source Squares")
                .with_description("Number of squares for source series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(45)
                .with_category("Parasitics"),
        );
        if bulk == Bulk::Soi {
            // Body squares (order 46). Only the native B3SOI builder reads
            // NRB; a bulk device has no body-contact sheet resistance.
            sheet.add(
                PropertyDefinition::new("nrb")
                    .with_display_name("Body Squares")
                    .with_description("Number of squares for the SOI body-contact resistance")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(0.0))
                    .with_order(46)
                    .with_category("Parasitics"),
            );
        }

        // =========================================================================
        // Stress Category (order 60-79) - STI stress effects for advanced nodes
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("sa")
                .with_display_name("SA Distance")
                .with_description("Distance from gate edge to STI on source side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(60)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sb")
                .with_display_name("SB Distance")
                .with_description("Distance from gate edge to STI on drain side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(61)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sd")
                .with_display_name("SD Spacing")
                .with_description("Source-drain spacing for multi-finger devices")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(62)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sc")
                .with_display_name("SC Distance")
                .with_description("Distance to the nearest well edge for well-proximity effect")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(63)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sca")
                .with_display_name("SCA")
                .with_description("Integral of first distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(64)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scb")
                .with_display_name("SCB")
                .with_description("Integral of second distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(65)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scc")
                .with_display_name("SCC")
                .with_description("Integral of third distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(66)
                .with_category("Stress"),
        );

        // =========================================================================
        // Mismatch Category (order 70-79) - per-instance offsets applied on
        // top of the binned model card, read by the native BSIM paths.
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("delvto")
                .with_display_name("Vth Offset")
                .with_description("Threshold voltage shift added to this instance only")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(70)
                .with_category("Mismatch"),
        );
        sheet.add(
            PropertyDefinition::new("mulu0")
                .with_display_name("Mobility Factor")
                .with_description("Low-field mobility multiplier applied to this instance only")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(71)
                .with_category("Mismatch"),
        );

        if bulk == Bulk::Soi {
            // Self-heating network (order 75-77). RTH0/CTH0 default to the
            // model card; NSEG splits the channel for the thermal solve.
            sheet.add(
                PropertyDefinition::new("rth0")
                    .with_display_name("Thermal Resistance")
                    .with_description("Self-heating thermal resistance; defaults to the model RTH0")
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression(""))
                    .with_unit("K/W")
                    .with_order(75)
                    .with_category("Self-Heating"),
            );
            sheet.add(
                PropertyDefinition::new("cth0")
                    .with_display_name("Thermal Capacitance")
                    .with_description(
                        "Self-heating thermal capacitance; defaults to the model CTH0",
                    )
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression(""))
                    .with_unit("J/K")
                    .with_order(76)
                    .with_category("Self-Heating"),
            );
            sheet.add(
                PropertyDefinition::new("nseg")
                    .with_display_name("Channel Segments")
                    .with_description(
                        "Number of channel segments used by the distributed body solve",
                    )
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(1.0))
                    .with_range(1.0, 100.0)
                    .with_order(77)
                    .with_category("Self-Heating"),
            );
            sheet.add(
                PropertyDefinition::new("frbody")
                    .with_display_name("Body Factor")
                    .with_description("Layout-dependent scaling on the body-resistance network")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(1.0))
                    .with_order(78)
                    .with_category("Self-Heating"),
            );
        }

        // =========================================================================
        // Temperature Category (order 80-89)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("temp")
                .with_display_name("Temperature")
                .with_description(
                    "Absolute device temperature; overrides the circuit temperature entirely",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(""))
                .with_unit("°C")
                .with_order(80)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(81)
                .with_category("Temperature"),
        );

        // =========================================================================
        // Initial Conditions Category (order 90-99)
        // =========================================================================
        // No OFF switch and no region hint: the BSIM4 builder records OFF
        // and then never reads it, and no MOS model consumes an operating
        // region, so either control would promise a convergence aid the
        // solver cannot deliver.
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(92)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(93)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vbs")
                .with_display_name("IC VBS")
                .with_description("Initial bulk-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(94)
                .with_category("Initial Conditions"),
        );
        if bulk == Bulk::Soi {
            // The substrate and body-contact initial conditions exist only
            // on the SOI terminals, and only the B3SOI builder allocates a
            // branch for them.
            sheet.add(
                PropertyDefinition::new("ic_ves")
                    .with_display_name("IC VES")
                    .with_description("Initial substrate-source voltage for transient analysis")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(0.0))
                    .with_unit("V")
                    .with_order(95)
                    .with_category("Initial Conditions"),
            );
            sheet.add(
                PropertyDefinition::new("ic_vps")
                    .with_display_name("IC VPS")
                    .with_description("Initial body-contact-source voltage for transient analysis")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(0.0))
                    .with_unit("V")
                    .with_order(96)
                    .with_category("Initial Conditions"),
            );
        }

        sheet
    }
}
