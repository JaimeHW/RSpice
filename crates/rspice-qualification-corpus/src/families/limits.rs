//! Failure-path and resource-limit families.
//!
//! Failure decks run under operating-point semantics (the fixed
//! `failure_path` mapping) and must fail with exactly the pinned stable
//! code -- a changed classification is an engine wire-surface change and
//! fails generation here. Resource decks run under transient semantics:
//! the over-budget cases exceed the engine's two-million-point analysis
//! budget by construction (accepted steps are bounded below by
//! `stop / tmax`, so the count cannot fall under the ceiling however the
//! solver subdivides). The engine enforces that budget up front and fails
//! fast as `engine.resource_limit`; the adapter's own series ceiling sits
//! behind it as defense in depth. The bounded-acceptance cases stay far
//! enough under the budget that their result artifacts also respect the
//! worker's 64 MiB results budget at roughly 85 bytes per CSV row.

use crate::capture::{CaseDraft, Expectation, Parameter, Probe};
use crate::families::physics::{Pulse, first_order_final};

fn ohm_parameter(value: f64) -> Vec<Parameter> {
    vec![Parameter {
        name: "r1",
        unit: "Ohm",
        value,
    }]
}

fn failure_case(id: &str, deck: &str, parameters: Vec<Parameter>, code: &'static str) -> CaseDraft {
    CaseDraft {
        id: id.to_owned(),
        primary_category: "convergence_and_failure",
        extra_categories: vec![],
        deck: deck.to_owned(),
        parameters,
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: Expectation::FailsWith(code),
    }
}

pub fn drafts() -> Vec<CaseDraft> {
    let mut drafts = Vec::new();

    // Malformed netlists must fail as parse errors, never as solver noise.
    // (A resistor line with NO value at all is deliberately absent here:
    // the parser currently accepts it and simulates a substituted value,
    // an engine leniency defect tracked separately. These decks are
    // malformed in ways no reading can accept.)
    drafts.push(failure_case(
        "fail.parse.001",
        "* unterminated source function\n\
         v1 in 0 pulse(0 5\n\
         r1 in 0 1k\n\
         .op\n\
         .end\n",
        ohm_parameter(1e3),
        "netlist.parse_error",
    ));
    // An undefined model reference resolves at circuit construction, not
    // in the grammar, so its classification is the circuit-error code.
    drafts.push(failure_case(
        "fail.model.001",
        "* resistor bound to a model that is never defined\n\
         v1 in 0 dc 5\n\
         r1 in out banana\n\
         .op\n\
         .end\n",
        ohm_parameter(0.0),
        "engine.circuit_error",
    ));
    drafts.push(failure_case(
        "fail.parse.003",
        "* voltage source with a missing node\n\
         v1 in\n\
         r1 in 0 2.2k\n\
         .op\n\
         .end\n",
        ohm_parameter(2.2e3),
        "netlist.parse_error",
    ));

    // A deck whose directives do not include the requested class.
    drafts.push(failure_case(
        "fail.directive.001",
        "* transient-only deck submitted for an operating point\n\
         v1 in 0 dc 5\n\
         r1 in 0 3.3k\n\
         .tran 1u 10u\n\
         .end\n",
        ohm_parameter(3.3e3),
        "analysis.directive_missing",
    ));
    drafts.push(failure_case(
        "fail.directive.002",
        "* ac-only deck submitted for an operating point\n\
         v1 in 0 dc 0 ac 1\n\
         r1 in 0 4.7k\n\
         .ac dec 10 10 1k\n\
         .end\n",
        ohm_parameter(4.7e3),
        "analysis.directive_missing",
    ));
    drafts.push(failure_case(
        "fail.directive.003",
        "* deck with no analysis directive at all\n\
         v1 in 0 dc 5\n\
         r1 in 0 5.6k\n\
         .end\n",
        ohm_parameter(5.6e3),
        "analysis.directive_missing",
    ));

    // (A current source into a floating capacitor node is deliberately
    // absent: the engine currently regularizes the singular system into a
    // silent 1e12 V "success" — tracked separately as a topology-check
    // defect. Once the engine refuses it, that deck belongs here.)
    drafts.push(failure_case(
        "fail.directive.004",
        "* noise-only deck submitted for an operating point\n\
         v1 in 0 dc 0 ac 1\n\
         r1 in out 9.1k\n\
         r2 out 0 9.1k\n\
         .noise v(out) v1 lin 3 100 10000\n\
         .end\n",
        ohm_parameter(9.1e3),
        "analysis.directive_missing",
    ));

    // Structurally unsolvable circuits: the classification must stay a
    // bounded engine code, not a crash or a fabricated result.
    drafts.push(failure_case(
        "fail.singular.002",
        "* two voltage sources forcing different values on one node\n\
         v1 a 0 dc 5\n\
         v2 a 0 dc 3\n\
         r1 a 0 6.8k\n\
         .op\n\
         .end\n",
        ohm_parameter(6.8e3),
        "engine.circuit_error",
    ));
    drafts.push(failure_case(
        "fail.singular.003",
        "* inductor shorting a voltage source at dc\n\
         v1 in 0 dc 5\n\
         l1 in 0 10m\n\
         .op\n\
         .end\n",
        vec![Parameter {
            name: "l1",
            unit: "H",
            value: 1e-2,
        }],
        "engine.circuit_error",
    ));

    // A voltage loop whose potentials cannot close: five volts around one
    // side, seven around the other.
    drafts.push(failure_case(
        "fail.singular.004",
        "* inconsistent voltage-source loop\n\
         v1 a 0 dc 5\n\
         v2 a b dc 3\n\
         v3 b 0 dc 4\n\
         r1 a 0 8.2k\n\
         .op\n\
         .end\n",
        ohm_parameter(8.2e3),
        "engine.circuit_error",
    ));

    // Over-budget transient runs: stop/tmax puts the accepted-sample count
    // past the two-million ceiling whatever the step controller does.
    for (id, r, c, stop, tmax) in [
        ("limits.series-budget.001", 1.0e3, 1.0e-6, 2.05, 1e-6),
        ("limits.series-budget.002", 2.0e3, 5.0e-7, 1.05, 5e-7),
        ("limits.series-budget.003", 3.3e3, 1.0e-7, 4.2, 2e-6),
    ] {
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "security_and_resource_limits",
            extra_categories: vec![],
            deck: format!(
                "* transient run exceeding the series sample budget\n\
                 v1 in 0 pulse(0 5 0.001 0.000001 0.000001 1 10)\n\
                 r1 in out {r}\n\
                 c1 out 0 {c}\n\
                 .tran {tmax} {stop} 0 {tmax}\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "c1",
                    unit: "F",
                    value: c,
                },
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: r,
                },
                Parameter {
                    name: "t.stop",
                    unit: "s",
                    value: stop,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::FailsWith("engine.resource_limit"),
        });
    }

    // Bounded acceptance: long runs that stay inside every budget and
    // still land on the exact settled value. The pulse settles within the
    // first percent of the window, so the tail is a pure hold.
    for (id, level, r, c, stop, tmax) in [
        ("limits.bounded-run.001", 5.0, 1.0e3, 1.0e-6, 0.2, 1e-6),
        ("limits.bounded-run.002", 3.3, 2.2e3, 1.0e-6, 0.15, 1e-6),
        ("limits.bounded-run.003", 12.0, 4.7e3, 4.7e-7, 0.12, 1e-6),
        ("limits.bounded-run.004", 1.8, 1.0e4, 1.0e-7, 0.2, 2e-6),
        ("limits.bounded-run.005", 9.0, 6.8e3, 2.2e-7, 0.4, 5e-6),
        ("limits.bounded-run.006", 2.2, 3.9e3, 1.0e-6, 0.18, 1e-6),
        ("limits.bounded-run.007", 7.0, 5.1e3, 2.2e-7, 0.3, 2e-6),
    ] {
        let pulse = Pulse {
            initial: 0.0,
            pulsed: level,
            delay: 1e-3,
            rise: 1e-6,
            fall: 1e-6,
            width: 10.0,
            period: 100.0,
        };
        let expected = first_order_final(&pulse.breakpoints(stop), 0.0, r * c, stop);
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "security_and_resource_limits",
            extra_categories: vec![],
            deck: format!(
                "* long bounded transient run inside every budget\n\
                 v1 in 0 {source}\n\
                 r1 in out {r}\n\
                 c1 out 0 {c}\n\
                 .tran {tmax} {stop} 0 {tmax}\n\
                 .end\n",
                source = pulse.spice(),
            ),
            parameters: vec![
                Parameter {
                    name: "c1",
                    unit: "F",
                    value: c,
                },
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: r,
                },
                Parameter {
                    name: "t.stop",
                    unit: "s",
                    value: stop,
                },
                Parameter {
                    name: "v.pulsed",
                    unit: "V",
                    value: level,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![Probe {
                name: "v(out)".to_owned(),
                unit: "V",
                expected,
                absolute_tolerance: "1e-9",
                relative_tolerance: "1e-6",
            }]),
        });
    }

    // Two additional over-budget shapes: an RL current path and a loaded
    // divider, proving the ceiling is topology-independent.
    drafts.push(CaseDraft {
        id: "limits.series-budget.004".to_owned(),
        primary_category: "security_and_resource_limits",
        extra_categories: vec![],
        deck: "* rl transient run exceeding the series sample budget\n\
               v1 in 0 pulse(0 5 0.001 0.000001 0.000001 1 10)\n\
               r1 in out 100\n\
               l1 out 0 10m\n\
               .tran 0.000001 2.1 0 0.000001\n\
               .end\n"
            .to_owned(),
        parameters: vec![
            Parameter {
                name: "l1",
                unit: "H",
                value: 1e-2,
            },
            Parameter {
                name: "r1",
                unit: "Ohm",
                value: 1e2,
            },
        ],
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: Expectation::FailsWith("engine.resource_limit"),
    });
    drafts.push(CaseDraft {
        id: "limits.series-budget.005".to_owned(),
        primary_category: "security_and_resource_limits",
        extra_categories: vec![],
        deck: "* divider transient run exceeding the series sample budget\n\
               v1 in 0 pulse(0 4 0.001 0.000001 0.000001 1 10)\n\
               r1 in out 2k\n\
               r2 out 0 2k\n\
               c1 out 0 0.0000001\n\
               .tran 0.000001 2.02 0 0.000001\n\
               .end\n"
            .to_owned(),
        parameters: vec![
            Parameter {
                name: "r1",
                unit: "Ohm",
                value: 2e3,
            },
            Parameter {
                name: "r2",
                unit: "Ohm",
                value: 2e3,
            },
        ],
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: Expectation::FailsWith("engine.resource_limit"),
    });

    drafts
}
