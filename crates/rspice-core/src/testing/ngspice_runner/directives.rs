//! Ngspice analysis-directive parsing helpers.

use super::*;

impl TestRunner {
    /// Parse analysis directives from netlist source
    pub(super) fn parse_analyses(&self, source: &str) -> Vec<AnalysisSpec> {
        self.parse_analyses_with_control(source, true)
    }

    pub(super) fn parse_analyses_with_control(
        &self,
        source: &str,
        include_control_block_commands: bool,
    ) -> Vec<AnalysisSpec> {
        let mut analyses = Vec::new();
        let mut in_control_block = false;

        for line in source.lines() {
            let trimmed = Self::strip_netlist_comment(line).trim();
            if trimmed.is_empty() {
                continue;
            }

            let normalized = trimmed.to_ascii_lowercase();
            if normalized == ".control" {
                in_control_block = true;
                continue;
            }
            if normalized == ".endc" {
                in_control_block = false;
                continue;
            }
            if in_control_block && !include_control_block_commands {
                continue;
            }

            if let Some(spec) = self.parse_analysis_line(&normalized, in_control_block) {
                analyses.push(spec);
            }
        }

        analyses
    }

    pub(super) fn parse_analysis_line(
        &self,
        line: &str,
        in_control_block: bool,
    ) -> Option<AnalysisSpec> {
        let normalized = if in_control_block && !line.starts_with('.') {
            format!(".{line}")
        } else {
            line.to_string()
        };

        // Match directive tokens exactly to avoid prefix collisions
        // (e.g., ".options" must not be treated as ".op").
        let directive = normalized.split_whitespace().next().unwrap_or("");
        match directive {
            ".op" => Some(AnalysisSpec::DcOp),
            ".dc" => self.parse_dc_directive(&normalized),
            ".tran" => self.parse_tran_directive(&normalized),
            ".ac" => self.parse_ac_directive(&normalized),
            ".disto" => self.parse_disto_directive(&normalized),
            ".pz" => self.parse_pz_directive(&normalized),
            ".noise" => self.parse_noise_directive(&normalized),
            ".sens" => self.parse_sensitivity_directive(&normalized),
            ".tf" => self.parse_transfer_directive(&normalized),
            ".four" | ".fourier" => Some(AnalysisSpec::Unsupported {
                directive: directive.trim_start_matches('.').to_ascii_uppercase(),
            }),
            _ => None,
        }
    }

    /// Parse .dc directive: .dc source start stop step
    pub(super) fn parse_dc_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 9 {
            let inner_source = parts[1].to_string();
            let inner_start = self.parse_spice_value(parts[2])?;
            let inner_stop = self.parse_spice_value(parts[3])?;
            let inner_step = self.parse_spice_value(parts[4])?;
            let outer_source = parts[5].to_string();
            let outer_start = self.parse_spice_value(parts[6])?;
            let outer_stop = self.parse_spice_value(parts[7])?;
            let outer_step = self.parse_spice_value(parts[8])?;
            Some(AnalysisSpec::DcSweep2 {
                inner_source,
                inner_start,
                inner_stop,
                inner_step,
                outer_source,
                outer_start,
                outer_stop,
                outer_step,
            })
        } else if parts.len() >= 5 {
            let source = parts[1].to_string();
            let start = self.parse_spice_value(parts[2])?;
            let stop = self.parse_spice_value(parts[3])?;
            let step = self.parse_spice_value(parts[4])?;
            Some(AnalysisSpec::DcSweep {
                source,
                start,
                stop,
                step,
            })
        } else {
            None
        }
    }

    /// Parse .tran directive: .tran tstep tstop [tstart] [tmax]
    pub(super) fn parse_tran_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let tstep = self.parse_spice_value(parts[1])?;
            let tstop = self.parse_spice_value(parts[2])?;
            let tstart = if parts.len() > 3 {
                self.parse_spice_value(parts[3]).unwrap_or(0.0)
            } else {
                0.0
            };
            let tmax = if parts.len() > 4 {
                self.parse_spice_value(parts[4])
            } else {
                None
            };
            Some(AnalysisSpec::Transient {
                tstep,
                tstop,
                tstart,
                tmax,
            })
        } else {
            None
        }
    }

    /// Parse .ac directive: .ac dec|oct|lin points fstart fstop
    pub(super) fn parse_ac_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let sweep_type = match parts[1] {
                "dec" => AcSweepType::Dec,
                "oct" => AcSweepType::Oct,
                "lin" => AcSweepType::Lin,
                _ => return None,
            };
            let points = parts[2].parse().ok()?;
            let fstart = self.parse_spice_value(parts[3])?;
            let fstop = self.parse_spice_value(parts[4])?;
            Some(AnalysisSpec::Ac {
                sweep_type,
                points,
                fstart,
                fstop,
            })
        } else {
            None
        }
    }

    /// Parse .disto directive using AC sweep semantics:
    /// .disto dec|oct|lin points fstart fstop [f2overf1]
    ///
    /// The ngspice runner currently validates against AC-like responses, so we
    /// map DISTO sweep setup onto AC execution for broad compatibility coverage.
    pub(super) fn parse_disto_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let sweep_type = match parts[1] {
                "dec" => AcSweepType::Dec,
                "oct" => AcSweepType::Oct,
                "lin" => AcSweepType::Lin,
                _ => return None,
            };
            let points = parts[2].parse().ok()?;
            let fstart = self.parse_spice_value(parts[3])?;
            let fstop = self.parse_spice_value(parts[4])?;
            Some(AnalysisSpec::Ac {
                sweep_type,
                points,
                fstart,
                fstop,
            })
        } else {
            None
        }
    }

    /// Parse .pz directive: .pz in+ in- out+ out- cur|vol pol|zer|pz
    pub(super) fn parse_pz_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return None;
        }

        let input_is_current = match parts[5] {
            "cur" => true,
            "vol" => false,
            _ => return None,
        };
        let (compute_poles, compute_zeros) = match parts[6] {
            "pol" | "pole" | "poles" => (true, false),
            "zer" | "zero" | "zeros" => (false, true),
            "pz" => (true, true),
            _ => return None,
        };

        Some(AnalysisSpec::PoleZero {
            input_pos: parts[1].to_string(),
            input_neg: Self::optional_probe_node(parts[2]),
            output_pos: parts[3].to_string(),
            output_neg: Self::optional_probe_node(parts[4]),
            input_is_current,
            compute_poles,
            compute_zeros,
        })
    }

    /// Parse .noise directive: .noise v(out[,ref]) input dec|oct|lin points fstart fstop
    pub(super) fn parse_noise_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return None;
        }

        let (output_pos, output_neg) = Self::parse_voltage_probe(parts[1])?;
        let sweep_type = match parts[3] {
            "dec" => AcSweepType::Dec,
            "oct" => AcSweepType::Oct,
            "lin" => AcSweepType::Lin,
            _ => return None,
        };
        let points = parts[4].parse().ok()?;
        let fstart = self.parse_spice_value(parts[5])?;
        let fstop = self.parse_spice_value(parts[6])?;

        Some(AnalysisSpec::Noise {
            output_pos,
            output_neg,
            input_source: parts[2].to_string(),
            sweep_type,
            points,
            fstart,
            fstop,
        })
    }

    pub(super) fn parse_sensitivity_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        let (output_pos, output_neg) = Self::parse_voltage_probe(parts[1])?;
        let sweep = if parts.len() == 2 {
            None
        } else {
            if parts.len() < 7 || parts[2] != "ac" {
                return Some(AnalysisSpec::Unsupported {
                    directive: "SENS".to_string(),
                });
            }
            let sweep_type = match parts[3] {
                "dec" => AcSweepType::Dec,
                "oct" => AcSweepType::Oct,
                "lin" => AcSweepType::Lin,
                _ => return None,
            };
            let points = parts[4].parse().ok()?;
            let fstart = self.parse_spice_value(parts[5])?;
            let fstop = self.parse_spice_value(parts[6])?;
            Some((sweep_type, points, fstart, fstop))
        };

        Some(AnalysisSpec::Sensitivity {
            output_pos,
            output_neg,
            sweep,
        })
    }

    pub(super) fn parse_transfer_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        Some(AnalysisSpec::TransferFunction {
            output: parts[1].to_string(),
            input_source: parts[2].to_string(),
        })
    }

    /// Parse SPICE values using the same lexer/parser as netlist parsing.
    /// This keeps regression analysis directives aligned with production parsing
    /// (e.g. 1ns, 10us, 10ghz, 1nF, 1mH).
    pub(super) fn parse_spice_value(&self, s: &str) -> Option<Value> {
        crate::netlist::lexer::parse_spice_value(s).ok()
    }

    /// Check if netlist uses unsupported features
    pub(super) fn check_unsupported(&self, source: &str) -> Option<String> {
        // Directive-based patterns.
        // NOTE: .subckt is now supported via flattening.
        // NOTE: regression analyses are dispatched explicitly; unsupported
        //       requests fail loudly in `run_test`.
        // NOTE: `.control` scripting is not interpreted here. Decks that rely
        //       on upstream control scripts must be explicitly declared in
        //       `validation-manifest.tsv` so they never pass silently.
        let directive_patterns: [(&str, &str); 0] = [];

        // Device-based patterns (must be at start of line)
        // These are single-letter device prefixes that need line-start checking
        // NOTE: t (transmission line), k (coupled inductors), b (behavioral),
        //       s (vswitch), w (iswitch) are now supported and NOT skipped
        let line_start_devices: [(&str, &str); 0] = [
            // All previously listed devices are now supported
        ];

        for line in source.lines() {
            let trimmed = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.is_empty() {
                continue;
            }

            if let Some(directive) = trimmed.strip_prefix('.') {
                let token = format!(".{}", directive.split_whitespace().next().unwrap_or(""));
                if token == ".model"
                    && let Some(model) = Self::unsupported_xspice_digital_event_model(&trimmed)
                {
                    return Some(format!(
                        "XSPICE digital event models such as '{}' require the digital event kernel, which is not implemented yet",
                        model
                    ));
                }
                for (pattern, reason) in directive_patterns {
                    if token == pattern {
                        return Some(reason.to_string());
                    }
                }
            }

            // Check if line starts with device letter followed by alphanumeric (e.g., "t1", "k1")
            for (prefix, reason) in &line_start_devices {
                if trimmed.starts_with(prefix) {
                    // Verify next char is alphanumeric (device name) not just a word
                    if let Some(next_char) = trimmed.chars().nth(1)
                        && next_char.is_alphanumeric()
                    {
                        return Some(reason.to_string());
                    }
                }
            }
        }

        None
    }

    fn unsupported_xspice_digital_event_model(line: &str) -> Option<&'static str> {
        let mut parts = line.split_whitespace();
        if !parts.next()?.eq_ignore_ascii_case(".model") {
            return None;
        }
        let _model_name = parts.next()?;
        if let Some(model_type) = parts.next()
            && let Some(model) = Self::unsupported_xspice_digital_event_model_name(model_type)
        {
            return Some(model);
        }

        for token in line.split(|ch: char| ch.is_whitespace() || ch == '(' || ch == ')') {
            let Some((key, value)) = token.split_once('=') else {
                continue;
            };
            if key.eq_ignore_ascii_case("model")
                && let Some(model) = Self::unsupported_xspice_digital_event_model_name(value)
            {
                return Some(model);
            }
        }

        None
    }

    fn unsupported_xspice_digital_event_model_name(token: &str) -> Option<&'static str> {
        let normalized = token
            .trim_matches(|ch| ch == '"' || ch == '\'' || ch == '(' || ch == ')')
            .to_ascii_lowercase();
        match normalized.as_str() {
            "d_source" => Some("d_source"),
            "d_state" => Some("d_state"),
            "d_ram" => Some("d_ram"),
            "d_rom" => Some("d_rom"),
            _ => None,
        }
    }

    pub(super) fn strip_netlist_comment(line: &str) -> &str {
        let no_inline = line.split_once(';').map(|(head, _)| head).unwrap_or(line);
        let trimmed = no_inline.trim_start();
        if trimmed.starts_with('*') || trimmed.starts_with("//") {
            ""
        } else {
            no_inline
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Suite Execution
    // ═══════════════════════════════════════════════════════════════════════════
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_detection_rejects_event_driven_xspice_digital_file_models() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let deck = "\
digital file model
a_source [a1 a2] d_source1
a_counter [up] clk reset [o1 o2] d_state1
.model d_source1 d_source (input_file=\"stimulus.txt\")
.model d_state1 d_state (state_file=\"states.txt\")
.end
";

        let reason = runner
            .check_unsupported(deck)
            .expect("event-driven XSPICE digital file models are not supported yet");

        assert!(
            reason.contains("XSPICE digital event models"),
            "unexpected unsupported reason: {reason}"
        );
    }
}
