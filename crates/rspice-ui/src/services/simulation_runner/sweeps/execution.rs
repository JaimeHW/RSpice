//! Turning a corner declaration into the concrete points it declares.
//!
//! Nothing here solves anything. Every declared point is solved by its own
//! prepared task, so what a declaration needs from this module is the list of
//! points it will have to answer for.

use super::super::error::{ServiceRunError, ServiceRunResult};
use super::types::{CornerPoint, CornerRunConfig};

/// Turn a corner contract into the exact points it declares.
///
/// Uncancellable on purpose: this runs during run preparation, before any
/// solve, and is bounded by the batch limit checked below.
pub(super) fn expand_corner_points(
    config: &CornerRunConfig,
    max_batch_runs: usize,
) -> ServiceRunResult<Vec<CornerPoint>> {
    // An explicit list is the space. It reaches here from a run set that
    // removed points from its cross product, and re-deriving the matrix from
    // the axes would put every one of them back. This is the only expansion
    // both the corner executor and operating-point PVT preparation call, so
    // honouring the list here is what keeps them running the same points.
    if !config.points.is_empty() {
        let requested = config.points.len();
        if requested > max_batch_runs {
            return Err(ServiceRunError::resource_limit(
                rspice_core::ResourceKind::BatchRuns,
                requested,
                max_batch_runs,
            ));
        }
        let mut points = Vec::new();
        points.try_reserve_exact(requested).map_err(|error| {
            ServiceRunError::Failure(format!(
                "Corner expansion allocation for {requested} explicit points failed: {error}"
            ))
        })?;
        points.extend(config.points.iter().copied());
        return Ok(points);
    }
    if config.process_corners.is_empty()
        || config.voltages.is_empty()
        || config.temperatures_c.is_empty()
    {
        return Err(ServiceRunError::Failure(
            "Corner expansion requires non-empty process, voltage, and temperature axes"
                .to_string(),
        ));
    }
    let requested = if config.full_matrix {
        config
            .process_corners
            .len()
            .checked_mul(config.voltages.len())
            .and_then(|count| count.checked_mul(config.temperatures_c.len()))
            .unwrap_or(usize::MAX)
    } else {
        config
            .process_corners
            .len()
            .max(config.voltages.len())
            .max(config.temperatures_c.len())
    };
    if requested > max_batch_runs {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::BatchRuns,
            requested,
            max_batch_runs,
        ));
    }
    let mut points = Vec::new();
    points.try_reserve_exact(requested).map_err(|error| {
        ServiceRunError::Failure(format!(
            "Corner expansion allocation for {requested} points failed: {error}"
        ))
    })?;

    if config.full_matrix {
        for process in &config.process_corners {
            for &voltage in &config.voltages {
                for &temperature_c in &config.temperatures_c {
                    points.push(CornerPoint {
                        process: *process,
                        voltage,
                        temperature_c,
                    });
                }
            }
        }
        return Ok(points);
    }

    // Diagonal pairing. `validate` has already refused unequal non-scalar
    // lengths, so the modulus below only ever shares a single-valued axis
    // across the points — it never cycles a longer one back to its start.
    for idx in 0..requested {
        points.push(CornerPoint {
            process: config.process_corners[idx % config.process_corners.len()],
            voltage: config.voltages[idx % config.voltages.len()],
            temperature_c: config.temperatures_c[idx % config.temperatures_c.len()],
        });
    }
    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::super::types::CornerProcess;
    use super::*;

    #[test]
    fn full_corner_matrix_obeys_configured_batch_limit_before_allocation() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT, CornerProcess::FF],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![-40.0, 125.0],
            ..Default::default()
        };

        let result = expand_corner_points(&config, 7);

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::BatchRuns,
                    requested: 8,
                    limit: 7,
                }
            ))
        ));
    }

    #[test]
    fn an_explicit_point_list_is_expanded_verbatim_rather_than_recomposed() {
        let points = vec![
            CornerPoint {
                process: CornerProcess::TT,
                voltage: 0.9,
                temperature_c: 125.0,
            },
            CornerPoint {
                process: CornerProcess::TT,
                voltage: 1.1,
                temperature_c: -40.0,
            },
        ];
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![-40.0, 125.0],
            points: points.clone(),
            ..Default::default()
        };

        let expanded = expand_corner_points(&config, 64).expect("an explicit list expands");

        assert_eq!(
            expanded, points,
            "the axes would compose four points; the declaration asked for these two"
        );
    }

    #[test]
    fn an_explicit_point_list_is_still_held_to_the_batch_limit() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            points: vec![
                CornerPoint {
                    process: CornerProcess::TT,
                    voltage: 1.0,
                    temperature_c: 25.0,
                };
                3
            ],
            ..Default::default()
        };

        let result = expand_corner_points(&config, 2);

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::BatchRuns,
                    requested: 3,
                    limit: 2,
                }
            ))
        ));
    }

    #[test]
    fn diagonal_corner_expansion_uses_largest_axis_as_run_count() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.0],
            temperatures_c: vec![-40.0, 25.0, 125.0],
            full_matrix: false,
            ..Default::default()
        };

        let points = expand_corner_points(&config, 3).expect("three diagonal points");

        assert_eq!(points.len(), 3);
        assert_eq!(points[2].voltage, 0.9);
        assert_eq!(points[2].temperature_c, 125.0);
    }
}
