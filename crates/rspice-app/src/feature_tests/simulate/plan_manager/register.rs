//! The omission register: every authored fact the port refused, and why.
//!
//! Six surfaces were rebuilt from one authored reference, and each wave decided
//! separately which of that reference's facts RSpice has an owner for. Those
//! decisions were right and they were also invisible: each lived in its own
//! module header, in prose, beside at most a local guard over its own route.
//! The predictable failure is not that someone disagrees — it is that someone
//! reads the authored reference, sees a column RSpice does not have, and adds
//! it back never having learned that it was considered and refused.
//!
//! So the reasons live here, once, in the order the reader meets the surfaces,
//! and the test at the bottom holds every one of them over every route. Each
//! entry is reconstructed from the route that refused it: nothing here is a
//! claim about the authored reference that the shipped code does not record.
//!
//! A child of the contract tests rather than a sibling of the routes: it reads
//! their render harness, and it is a statement about all six surfaces at once,
//! so it belongs to neither one of them.

// Native-only, like the render harness it reads.
#![cfg(not(target_arch = "wasm32"))]

use super::{EVERY_ROUTE, GATED_VIEWPORTS, RouteSpeech, route_speech};

/// One authored fact the port deliberately left behind, and what would tell us
/// it came back.
struct UnownedFact {
    /// The authored fact, in the words the refusing route uses for it.
    authored: &'static str,
    /// The surface it was authored on, named as the reader meets it.
    surface: &'static str,
    /// Why RSpice has no owner for it. Not "we ran out of room": a fact with no
    /// owner is one this product cannot state truthfully at all. Where the
    /// reason is something else — duplication, or one reachable option — the
    /// entry says so rather than borrowing the stronger word.
    reason: &'static str,
    /// What the guard looks for. Empty for a refusal that is not of a word — a
    /// rendering, or an arithmetic — where the reason names what holds it
    /// instead, because there is nothing a vocabulary scan could see.
    forbidden: &'static [Forbidden],
}

/// How a returning authored fact would be recognised.
///
/// The distinction is why this register can be trusted. A one-word heading
/// matched as a substring fires on any sentence that happens to use the word,
/// and a guard that fires on a sentence is a guard the next reader deletes. So
/// a word with other legitimate senses in this crate is matched as a whole
/// string, and only a phrase specific enough to mean one thing is matched
/// loosely.
///
/// There is deliberately no exception list. If a term turns out to be
/// legitimately present in another sense, the term is narrowed — an allowlist
/// beside a guard is how the guard stops meaning anything.
enum Forbidden {
    /// The whole painted string or announced value, compared exactly. For a
    /// column heading or an option label, where the authored fact is the name
    /// of a control rather than a claim inside a sentence.
    Label(&'static str),
    /// A phrase, found anywhere, compared without regard to case. Only for
    /// wording that means the authored fact and nothing else.
    Phrase(&'static str),
}

/// Every authored fact the six surfaces refused, with the reason it was refused
/// for.
///
/// Roughly a third of the authored reference did not survive contact with the
/// product, and that is not a gap to be closed later: an execution target
/// belongs to a run, a plan carries no modified stamp and no design binding, and
/// there is no governance model here to waive, approve or release against.
/// Adding any of these back means adding the concept underneath it first.
const UNOWNED_AUTHORED_FACTS: &[UnownedFact] = &[
    // -- Browse: the records table, its controls and its aside ---------------
    UnownedFact {
        authored: "the design and testbench binding column",
        surface: "Browse · records table",
        reason: "No plan binds a design or a testbench. The column is also one \
                 of the two that make the authored table 899 points wide inside \
                 a 685-point cell, so dropping it closes the fact audit and the \
                 fit in one move.",
        forbidden: &[
            Forbidden::Label("Design / testbench binding"),
            Forbidden::Label("Testbench"),
            Forbidden::Phrase("testbench binding"),
        ],
    },
    UnownedFact {
        authored: "the modified column",
        surface: "Browse · records table",
        reason: "Nothing stamps a plan as modified. A column of blanks, or of \
                 the load time, is a timestamp the reader would date work by.",
        forbidden: &[Forbidden::Label("Modified")],
    },
    UnownedFact {
        authored: "the filter placeholder's binding field",
        surface: "Browse · filter",
        reason: "A binding is not a thing an RSpice plan has, so a placeholder \
                 naming it sends the reader looking for a plan by a word the \
                 filter can never match.",
        forbidden: &[Forbidden::Phrase("plan, binding")],
    },
    UnownedFact {
        authored: "the Governed baselines scope",
        surface: "Browse · scope",
        reason: "RSpice has no governance state on a plan, so the option would \
                 select an empty set forever.",
        forbidden: &[
            Forbidden::Label("Governed baselines"),
            Forbidden::Phrase("governed baseline"),
        ],
    },
    UnownedFact {
        authored: "the boundary note's dirty-editor clause",
        surface: "Browse · boundary notes",
        reason: "Nothing in this module checks for an open editor before \
                 switching plans, so the clause would describe a check that does \
                 not run.",
        forbidden: &[Forbidden::Phrase("dirty editor")],
    },
    UnownedFact {
        authored: "the boundary note's permission clause",
        surface: "Browse · boundary notes",
        reason: "RSpice refuses a switch on a validation failure, not on a \
                 permission. There is no permission model to refuse against.",
        forbidden: &[Forbidden::Phrase("permission")],
    },
    UnownedFact {
        authored: "the boundary note's entitlement-failure clause",
        surface: "Browse · boundary notes",
        reason: "Nothing here classifies an entitlement, so an entitlement \
                 failure cannot happen and must not be listed among the things \
                 that might.",
        forbidden: &[Forbidden::Phrase("entitlement")],
    },
    UnownedFact {
        authored: "the boundary note's schema-migration clause",
        surface: "Browse · boundary notes",
        reason: "Switching plans migrates no schema; the catalog holds one shape \
                 and the switch is atomic within it.",
        forbidden: &[Forbidden::Phrase("schema migration")],
    },
    UnownedFact {
        authored: "the aside's per-plan design and testbench binding row",
        surface: "Browse · selected plan",
        reason: "The same missing owner as the column: a plan binds neither. The \
                 aside states the strongest facts the catalog does own instead \
                 of stubbing these two.",
        forbidden: &[Forbidden::Phrase("design and testbench binding")],
    },
    UnownedFact {
        authored: "the aside's named execution profile row",
        surface: "Browse · selected plan",
        reason: "No plan owns an execution profile. Execution is chosen when a \
                 run is dispatched, not declared on the plan.",
        forbidden: &[
            Forbidden::Label("Execution profile"),
            Forbidden::Phrase("execution profile"),
        ],
    },
    // -- Rename --------------------------------------------------------------
    UnownedFact {
        authored: "the deprecated-name alias policy",
        surface: "Rename",
        reason: "There is no alias field on a plan anywhere in the catalog, so \
                 the control would offer a choice with no effect.",
        forbidden: &[Forbidden::Phrase("alias")],
    },
    UnownedFact {
        authored: "the count of name-based script references",
        surface: "Rename",
        reason: "It would report the result of an audit nothing in RSpice \
                 performs — and a zero there asserts the audit ran and found \
                 none, which is the worse of the two lies.",
        // Not a bare "require review": Import paints `Imported model bindings
        // require review: …` when a package's closure cannot be resolved, and
        // that is a real refusal about something RSpice does own. The authored
        // line counts *references*, which is the half with no owner.
        forbidden: &[
            Forbidden::Phrase("name-based script"),
            Forbidden::Phrase("references require review"),
        ],
    },
    // -- Create --------------------------------------------------------------
    UnownedFact {
        authored: "the starting-point selector",
        surface: "Create",
        reason: "None of its four options survives: one has no owner, two \
                 already have their own door, and the fourth no longer describes \
                 what this route makes. A selector with one reachable option is \
                 not a control.",
        forbidden: &[Forbidden::Phrase("starting point")],
    },
    UnownedFact {
        authored: "the Organization template starting point",
        surface: "Create",
        reason: "RSpice has no organization policy engine and no org-managed \
                 template library, so the option would name a source that does \
                 not exist.",
        forbidden: &[
            Forbidden::Label("Organization template"),
            Forbidden::Phrase("organization template"),
        ],
    },
    UnownedFact {
        authored: "the selected template digest the authored table cites",
        surface: "Create",
        reason: "With no template library there is nothing to digest, so the \
                 value would be a digest of nothing.",
        forbidden: &[Forbidden::Phrase("template digest")],
    },
    UnownedFact {
        authored: "the Clone active plan setup starting point",
        surface: "Create",
        reason: "Not an ownership gap: Clone already has its own door, draft and \
                 transaction on the selected row. A third route onto a reachable \
                 transaction is the clutter this product's bar rules out, and \
                 `copy_analyses_options` cannot even express the inheritance \
                 this route offers.",
        forbidden: &[Forbidden::Phrase("clone active plan setup")],
    },
    UnownedFact {
        authored: "the import resolved manifest starting point",
        surface: "Create",
        reason: "Not an ownership gap: Import has its own door in the toolbar \
                 and its own transaction. Nothing in RSpice calls a package a \
                 resolved manifest either.",
        forbidden: &[Forbidden::Phrase("resolved manifest")],
    },
    UnownedFact {
        authored: "the Blank plan starting point",
        surface: "Create",
        reason: "Not an ownership gap: it is the only option left, and a plan \
                 created here carries a chosen reference point and up to three \
                 inherited domains — so the authored blank-or-clone dichotomy \
                 describes neither end of what this route makes.",
        forbidden: &[
            Forbidden::Label("Blank plan"),
            Forbidden::Phrase("blank plan"),
        ],
    },
    UnownedFact {
        authored: "the exclusion table's waivers row",
        surface: "Create · not copied",
        reason: "This product has no waivers, and an exclusion is only a fact \
                 about something the product could otherwise have copied.",
        forbidden: &[Forbidden::Phrase("waiver")],
    },
    UnownedFact {
        authored: "the exclusion table's approvals row",
        surface: "Create · not copied",
        reason: "There is no approval system here, so the row would assert a \
                 governance model that does not exist.",
        forbidden: &[Forbidden::Phrase("approval")],
    },
    UnownedFact {
        authored: "the exclusion table's release evidence row",
        surface: "Create · not copied",
        reason: "There is no release evidence to withhold.",
        forbidden: &[Forbidden::Phrase("release evidence")],
    },
    // -- Compare -------------------------------------------------------------
    UnownedFact {
        authored: "the execution binding and target comparison domain",
        surface: "Compare",
        reason: "Neither half exists: no plan binds a design or a testbench, and \
                 `SimulationRun::execution_target` is a property of a run, \
                 chosen when that run is dispatched. Painting the row as \
                 unchanged would assert a comparison that never ran, so the \
                 domain is dropped.\n\
                 The route used to say so on the surface, in a note beside the \
                 table. It no longer does: a dialog that describes what it does \
                 not have is describing it to a reader with no way of knowing it \
                 was ever offered. This entry is where the absence is held now — \
                 over every route, in both channels, at every gated viewport — \
                 alongside `the_execution_domain_is_absent_and_the_surface_does_\
                 not_narrate_it`, which holds the route's own domain set at four.",
        forbidden: &[
            Forbidden::Label("Execution binding and target"),
            Forbidden::Phrase("execution binding"),
        ],
    },
    // -- Exchange ------------------------------------------------------------
    UnownedFact {
        authored: "the package-format choice",
        surface: "Export",
        reason: "There is one format, so the control would state a choice the \
                 reader does not have.",
        forbidden: &[
            Forbidden::Label("Package format"),
            Forbidden::Phrase("format choice"),
        ],
    },
    UnownedFact {
        authored: "the plan-revision choice",
        surface: "Export",
        reason: "The export is of the plan as the catalog holds it. No earlier \
                 revision is retained to export instead.",
        forbidden: &[Forbidden::Phrase("revision choice")],
    },
    UnownedFact {
        authored: "the reference policy",
        surface: "Export",
        reason: "Nothing is embedded by reference, so there is no policy to \
                 choose between.",
        forbidden: &[Forbidden::Phrase("reference policy")],
    },
    UnownedFact {
        authored: "the redaction policy",
        surface: "Export",
        reason: "Nothing a plan holds carries a classification, so there is \
                 nothing to redact.",
        forbidden: &[Forbidden::Phrase("redaction")],
    },
    UnownedFact {
        authored: "the digest inventory",
        surface: "Export",
        reason: "RSpice signs nothing. The route says so instead, which is the \
                 honest form of that claim.",
        forbidden: &[Forbidden::Phrase("digest inventory")],
    },
    UnownedFact {
        authored: "the signature verification stage",
        surface: "Export",
        reason: "There is nothing to verify a signature against.",
        forbidden: &[Forbidden::Phrase("signature verification")],
    },
    UnownedFact {
        authored: "the validation table's digest and signature check",
        surface: "Import",
        reason: "Has no owner in RSpice, and is not restated here as a thing \
                 that might happen. The two stages that do have owners are \
                 listed in the order the transaction runs them.",
        forbidden: &[Forbidden::Phrase("signature check")],
    },
    UnownedFact {
        authored: "the validation table's capability and entitlement \
                   classification",
        surface: "Import",
        reason: "Nothing here classifies a capability or an entitlement, so the \
                 stage would be a check that never runs.",
        forbidden: &[Forbidden::Phrase("capability and entitlement")],
    },
    UnownedFact {
        authored: "the per-stage failure column",
        surface: "Import",
        reason: "Not an ownership gap and not a word: every stage fails the same \
                 way, so the authored column spends a third of its width \
                 restating one sentence four times. What holds it is the fit \
                 gate — a fifth column does not fit the narrowest gated viewport \
                 — and the route's own refusal tests, which show one refusal \
                 path rather than four.",
        forbidden: &[],
    },
    UnownedFact {
        authored: "the package JSON painted in the dialog",
        surface: "Export and Import",
        reason: "Not a word either: a package is a file the reader chooses, not \
                 text copied between two windows, and painting its JSON would \
                 put the one thing the route exists to carry below the fold of a \
                 dialog that may not scroll. What holds it is the fit gate, \
                 which a painted package fails outright.",
        forbidden: &[],
    },
    // -- Campaign ------------------------------------------------------------
    UnownedFact {
        authored: "the shared target queue, and the execution target control \
                   under it",
        surface: "Campaign",
        reason: "The declared-order half is real and is the table's Order \
                 column. The queue is shared with nothing — the controller \
                 refuses to start while any run or campaign is active — and no \
                 campaign owns an execution target, because \
                 `prepare_and_start_campaign` takes no target parameter. So the \
                 authored target control is not ported rather than stubbed.",
        forbidden: &[Forbidden::Phrase("target queue")],
    },
    UnownedFact {
        authored: "the per-plan manifests and the campaign summary record",
        surface: "Campaign",
        reason: "The per-member half holds, but nothing in RSpice calls that a \
                 manifest and there is no campaign summary record at all: the \
                 only thing that outlives the dispatch is the membership written \
                 onto each member's own run.",
        forbidden: &[
            Forbidden::Phrase("manifest"),
            Forbidden::Phrase("campaign summary"),
        ],
    },
    UnownedFact {
        authored: "the claim that a failing member never cancels its siblings",
        surface: "Campaign",
        reason: "True only once dispatch has begun. Preparation walks the \
                 members with `?`, so a member that cannot be prepared refuses \
                 the whole campaign and nothing is queued — and the reader is \
                 deciding at the moment the strict half applies. The surface \
                 states both halves rather than the authored one.",
        forbidden: &[Forbidden::Phrase("never cancels")],
    },
];

/// No route says any of the authored vocabulary that has no owner here.
///
/// One test over every route and every entry, rather than a guard per wave.
/// Six local guards would each cover the surface their author was looking at,
/// which is exactly the shape of the defect: the reason a column was refused on
/// Browse is the reason it must not appear on Compare either, and the wave that
/// adds the seventh surface inherits all thirty-odd decisions without reading
/// six module headers.
///
/// Both channels are scanned. Painting a refused fact and announcing one are the
/// same regression to a reader who cannot see the screen.
///
/// All three gated viewports, because a route may state something in the stacked
/// arrangement that it does not state in the two-column one.
///
/// What this does not reach is a route's refusal text: the fixtures are valid, so
/// no validation message is painted, and a phrase chosen here has to be one that
/// would still be wrong in a refusal. `Imported model bindings require review` is
/// the reason the rename entry counts references rather than reviews.
#[test]
fn no_route_states_an_authored_fact_that_has_no_owner() {
    assert!(
        UNOWNED_AUTHORED_FACTS
            .iter()
            .any(|fact| !fact.forbidden.is_empty()),
        "the register forbids no vocabulary at all, so this test proves nothing"
    );
    for mode in EVERY_ROUTE {
        for (arrangement, screen) in GATED_VIEWPORTS {
            let RouteSpeech { painted, announced } = route_speech(screen, mode);
            assert!(
                !painted.is_empty(),
                "{mode:?} in {arrangement} painted nothing at all, so this gate \
                 would pass whatever it said"
            );
            let said = painted.iter().chain(announced.iter());
            let lowered = said
                .clone()
                .map(|text| text.to_ascii_lowercase())
                .collect::<Vec<_>>();
            for fact in UNOWNED_AUTHORED_FACTS {
                for forbidden in fact.forbidden {
                    let (found, spelling) = match forbidden {
                        Forbidden::Label(label) => (
                            said.clone().any(|text| text == label),
                            format!("the label {label:?}"),
                        ),
                        Forbidden::Phrase(phrase) => {
                            let phrase = phrase.to_ascii_lowercase();
                            (
                                lowered.iter().any(|text| text.contains(&phrase)),
                                format!("the phrase {phrase:?}"),
                            )
                        }
                    };
                    assert!(
                        !found,
                        "{mode:?} in {arrangement} states {spelling}.\n\
                         That is {}, authored on {} and not ported: {}\n\
                         If the product now owns it, delete the register entry \
                         in tests.rs and say so. If it does not, this surface is \
                         claiming something RSpice cannot back.",
                        fact.authored, fact.surface, fact.reason
                    );
                }
            }
        }
    }
}
