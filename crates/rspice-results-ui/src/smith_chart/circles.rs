//! The two-port circles a Smith sheet draws at one retained frequency.

use rspice_results::network_matrix::two_port::{
    MaximumGain, SmithCircle, TwoPortNoiseParameters, TwoPortScattering, TwoPortSweep,
};

use super::state::SmithCircleSet;

/// Which termination a circle constrains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CirclePlane {
    /// Γ_S, the source seen by port 1; referred to port 1's Z₀.
    Source,
    /// Γ_L, the load seen by port 2; referred to port 2's Z₀.
    Load,
}

/// What a circle is the locus of.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircleKind {
    Stability { stable_inside: bool },
    AvailableGain { decibels: f64 },
    PowerGain { decibels: f64 },
    NoiseFigure { decibels: f64 },
}

/// One circle to draw.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartCircle {
    pub kind: CircleKind,
    pub plane: CirclePlane,
    pub circle: SmithCircle,
}

impl ChartCircle {
    /// The tag the chart writes beside the circle.
    pub fn tag(&self) -> String {
        match self.kind {
            CircleKind::Stability { .. } => match self.plane {
                CirclePlane::Source => "source stab".to_owned(),
                CirclePlane::Load => "load stab".to_owned(),
            },
            CircleKind::AvailableGain { decibels } => format!("GA {}", decibel_text(decibels)),
            CircleKind::PowerGain { decibels } => format!("GP {}", decibel_text(decibels)),
            CircleKind::NoiseFigure { decibels } => format!("NF {}", decibel_text(decibels)),
        }
    }
}

/// `12 dB`, `1.5 dB`: as few digits as the value needs, up to two.
pub fn decibel_text(decibels: f64) -> String {
    let rounded = (decibels * 100.0).round() / 100.0;
    let text = format!("{rounded:.2}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    format!("{text} dB")
}

/// The two-port figures at the circles' frequency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoPortFigures {
    pub frequency: f64,
    pub index: usize,
    pub scattering: TwoPortScattering,
    pub reference_impedances_ohm: [f64; 2],
    pub noise: Option<TwoPortNoiseParameters>,
}

impl TwoPortFigures {
    pub fn at(sweep: &TwoPortSweep, index: usize) -> Option<Self> {
        Some(Self {
            frequency: *sweep.frequencies.get(index)?,
            index,
            scattering: *sweep.scattering.get(index)?,
            reference_impedances_ohm: sweep.reference_impedances_ohm,
            noise: sweep
                .noise
                .as_ref()
                .and_then(|noise| noise.get(index).copied()),
        })
    }

    /// The gain-circle values when the reader chose none: the whole
    /// decibels at and below the maximum gain, three of them.
    pub fn default_gain_db(&self) -> Vec<f64> {
        let Some(maximum) = self.scattering.maximum_gain() else {
            return Vec::new();
        };
        let top = (10.0 * maximum.linear().log10()).floor();
        (0..3).map(|step| top - f64::from(step)).collect()
    }

    /// The noise-circle values when the reader chose none: three
    /// half-decibel steps from the first half decibel above NFmin.
    pub fn default_noise_db(&self) -> Vec<f64> {
        let Some(noise) = self.noise else {
            return Vec::new();
        };
        let minimum = 10.0 * noise.minimum_noise_factor.log10();
        let first = (minimum * 2.0).floor() / 2.0 + 0.5;
        (0..3).map(|step| first + 0.5 * f64::from(step)).collect()
    }

    /// Every circle the set asks for that exists at this frequency, in
    /// drawing order: gain and noise under stability.
    pub fn circles(
        &self,
        set: SmithCircleSet,
        gain_db: &[f64],
        noise_db: &[f64],
    ) -> Vec<ChartCircle> {
        let device = self.scattering;
        let mut circles = Vec::new();
        let gains = if gain_db.is_empty() {
            self.default_gain_db()
        } else {
            gain_db.to_vec()
        };
        for decibels in &gains {
            let gain = 10f64.powf(decibels / 10.0);
            if set.available_gain
                && let Some(circle) = device.available_gain_circle(gain)
            {
                circles.push(ChartCircle {
                    kind: CircleKind::AvailableGain {
                        decibels: *decibels,
                    },
                    plane: CirclePlane::Source,
                    circle,
                });
            }
            if set.power_gain
                && let Some(circle) = device.power_gain_circle(gain)
            {
                circles.push(ChartCircle {
                    kind: CircleKind::PowerGain {
                        decibels: *decibels,
                    },
                    plane: CirclePlane::Load,
                    circle,
                });
            }
        }
        if set.noise
            && let Some(noise) = self.noise
        {
            let values = if noise_db.is_empty() {
                self.default_noise_db()
            } else {
                noise_db.to_vec()
            };
            for decibels in values {
                if let Some(circle) = noise.noise_circle(10f64.powf(decibels / 10.0)) {
                    circles.push(ChartCircle {
                        kind: CircleKind::NoiseFigure { decibels },
                        plane: CirclePlane::Source,
                        circle,
                    });
                }
            }
        }
        if set.stability {
            for (plane, circle) in [
                (CirclePlane::Source, device.source_stability_circle()),
                (CirclePlane::Load, device.load_stability_circle()),
            ] {
                if let Some(stability) = circle {
                    circles.push(ChartCircle {
                        kind: CircleKind::Stability {
                            stable_inside: stability.stable_inside,
                        },
                        plane,
                        circle: stability.circle,
                    });
                }
            }
        }
        circles
    }

    /// `MAG 14.6 dB` or `MSG 16.7 dB`.
    pub fn maximum_gain_text(&self) -> Option<(&'static str, String)> {
        let maximum = self.scattering.maximum_gain()?;
        let decibels = 10.0 * maximum.linear().log10();
        Some(match maximum {
            MaximumGain::Available(_) => ("MAG", format!("{decibels:.2} dB")),
            MaximumGain::Stable(_) => ("MSG", format!("{decibels:.2} dB")),
        })
    }
}

/// Parse a list of decibel values: `10 12 14`, `10, 12.5`. Refuses an empty
/// or non-finite entry so a typo never draws a different circle.
pub fn parse_decibels(text: &str) -> Result<Vec<f64>, String> {
    let mut values = Vec::new();
    for token in text
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|token| !token.is_empty())
    {
        let token = token.strip_suffix("dB").unwrap_or(token);
        let value = token
            .parse::<f64>()
            .map_err(|_| format!("'{token}' is not a number of decibels"))?;
        if !value.is_finite() {
            return Err(format!("'{token}' is not a finite number of decibels"));
        }
        values.push(value);
    }
    if values.len() > 8 {
        return Err("at most eight circles".to_owned());
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    fn device() -> TwoPortScattering {
        let polar = |m: f64, a: f64| Complex64::from_polar(m, a.to_radians());
        TwoPortScattering {
            s11: polar(0.533, 176.6),
            s12: polar(0.06, 58.4),
            s21: polar(2.800, 64.5),
            s22: polar(0.604, -58.3),
        }
    }

    fn figures(noise: bool) -> TwoPortFigures {
        TwoPortFigures {
            frequency: 1.4e9,
            index: 0,
            scattering: device(),
            reference_impedances_ohm: [50.0, 50.0],
            noise: noise.then_some(TwoPortNoiseParameters {
                minimum_noise_factor: 10f64.powf(0.16),
                noise_resistance_ohm: 20.0,
                optimum_source_reflection: Complex64::from_polar(0.62, 100f64.to_radians()),
                reference_impedance_ohm: 50.0,
            }),
        }
    }

    #[test]
    fn default_circles_sit_at_whole_decibels_below_the_maximum() {
        // MAG is 14.61 dB at 1.4 GHz.
        assert_eq!(figures(false).default_gain_db(), vec![14.0, 13.0, 12.0]);
        // NFmin is 1.6 dB.
        assert_eq!(figures(true).default_noise_db(), vec![2.0, 2.5, 3.0]);
        assert!(figures(false).default_noise_db().is_empty());
    }

    #[test]
    fn the_set_draws_exactly_the_circles_it_names() {
        let all = SmithCircleSet {
            stability: true,
            available_gain: true,
            power_gain: true,
            noise: true,
        };
        let circles = figures(true).circles(all, &[13.0], &[3.0]);
        let kinds = circles
            .iter()
            .map(|circle| (circle.tag(), circle.plane))
            .collect::<Vec<_>>();
        assert_eq!(
            kinds,
            vec![
                ("GA 13 dB".to_owned(), CirclePlane::Source),
                ("GP 13 dB".to_owned(), CirclePlane::Load),
                ("NF 3 dB".to_owned(), CirclePlane::Source),
                ("source stab".to_owned(), CirclePlane::Source),
                ("load stab".to_owned(), CirclePlane::Load),
            ]
        );
        // No noise circle is drawn without retained noise parameters.
        assert!(
            figures(false)
                .circles(all, &[13.0], &[3.0])
                .iter()
                .all(|circle| !matches!(circle.kind, CircleKind::NoiseFigure { .. }))
        );
        // A gain above the maximum has no circle rather than a wrong one.
        assert!(
            figures(false)
                .circles(
                    SmithCircleSet {
                        available_gain: true,
                        ..SmithCircleSet::default()
                    },
                    &[15.0],
                    &[]
                )
                .is_empty()
        );
    }

    #[test]
    fn decibel_lists_parse_strictly() {
        assert_eq!(parse_decibels("10 12.5, 14dB"), Ok(vec![10.0, 12.5, 14.0]));
        assert_eq!(parse_decibels("  "), Ok(Vec::new()));
        assert!(parse_decibels("10 twelve").is_err());
        assert!(parse_decibels("inf").is_err());
        assert_eq!(decibel_text(12.0), "12 dB");
        assert_eq!(decibel_text(1.5), "1.5 dB");
    }
}
