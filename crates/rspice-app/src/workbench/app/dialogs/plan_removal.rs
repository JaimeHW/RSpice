//! Confirming the removal of one record from the analysis plan.
//!
//! Removal is only staged when it costs something. A record that nothing
//! depends on and that has produced nothing is removed immediately — asking
//! about that would be friction, and friction is how a reader learns to
//! dismiss a dialog without reading it. What is worth stopping for is the
//! removal that silently orphans retained results, unbinds an analysis another
//! one needs to run at all, leaves an expression naming a variable the plan no
//! longer declares, or drops a capture policy the outputs under it were being
//! stored by.
//!
//! One review, four registries. The analysis stack asked before it removed and
//! the variable, output and capture-group registries did not, which made the
//! same irreversible class of edit — a plan mutation, not undoable from
//! schematic history — cost a confirmation on one page and a single click on
//! the next three.
//!
//! This module only renders and records the answer. The removal itself stays
//! with the page that owns the registry, which reads the confirmed answer on
//! its next frame.
//!
//! It also states the removals that are not questions. The plan refuses to
//! remove an analysis another one is bound to, on the same predicate the
//! surface can evaluate before opening anything — so that case was a
//! destructive review whose confirmation could only ever be answered by a
//! refusal. It is a notice now, with no primary action that destroys
//! anything and the blocking analyses named by the names they show under.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row};
use crate::workbench::RSpiceApp;

/// What one staged plan removal is about to take out of the plan.
///
/// Four registries, one review. Each variant carries the plan the review was
/// staged against as well as the record, so a confirmation answered while the
/// active plan changed underneath the modal applies to nothing rather than to
/// whatever now holds that position. The analysis stack is the exception only
/// because an analysis instance is identified plan-wide already.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanRemovalTarget {
    Analysis(crate::product::AnalysisInstanceId),
    Variable {
        plan: crate::product::SimulationPlanId,
        id: crate::product::DesignVariableId,
    },
    Output {
        plan: crate::product::SimulationPlanId,
        id: crate::product::SavedOutputId,
    },
    CaptureGroup {
        plan: crate::product::SimulationPlanId,
        id: crate::product::CaptureGroupId,
    },
}

/// How hard one consequence row's paragraph reads.
///
/// Not a color: `Warn` is what the reader loses and has to repair, `Aside` is
/// what survives removal and is said so they do not assume otherwise. A review
/// whose every line shouted would be a review nobody finished reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanRemovalTone {
    Warn,
    Aside,
}

/// One fact about what removal costs, resolved when the review opened.
///
/// The row is always stated — "none" is an answer a reader needs as much as a
/// list is — while the paragraph is only carried when there is something to
/// explain, which is what keeps a review of a cheap removal from reading like
/// a review of an expensive one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlanRemovalConsequence {
    pub(crate) fact: String,
    pub(crate) value: String,
    pub(crate) note: Option<(PlanRemovalTone, String)>,
}

impl PlanRemovalConsequence {
    /// A stated fact with nothing further to explain.
    pub(crate) fn stated(fact: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            fact: fact.into(),
            value: value.into(),
            note: None,
        }
    }

    pub(crate) fn explained(mut self, tone: PlanRemovalTone, note: impl Into<String>) -> Self {
        self.note = Some((tone, note.into()));
        self
    }
}

/// Destructive review for removing one record from the analysis plan.
///
/// Only raised where removal actually costs something: an analysis with
/// retained results or dependents, a variable the plan resolves into an
/// analysis, an output a capture group holds or a specification names, a
/// capture group that holds outputs. Removing a freshly-added record that
/// nothing depends on is not a decision worth a modal, and asking anyway
/// teaches the reader to dismiss the dialog without reading it.
///
/// Plan mutations are not undoable from schematic history, which is exactly
/// why the four registries share one review instead of three of them
/// committing on the first click.
#[derive(Debug, Clone, Default)]
pub(crate) struct PlanRemovalReview {
    pub(crate) target: Option<PlanRemovalTarget>,
    /// What removal takes with it, resolved when the review opened.
    pub(crate) label: String,
    pub(crate) consequences: Vec<PlanRemovalConsequence>,
    /// Set by the dialog when the reader confirms. The page that owns the
    /// registry performs the removal itself on its next frame, so the modal
    /// never reaches across into another surface's transactions.
    pub(crate) confirmed: bool,
}

impl PlanRemovalReview {
    /// Stage a review for one record.
    pub(crate) fn open(
        &mut self,
        target: PlanRemovalTarget,
        label: String,
        consequences: Vec<PlanRemovalConsequence>,
    ) {
        *self = Self {
            target: Some(target),
            label,
            consequences,
            confirmed: false,
        };
    }

    /// The analysis the reader confirmed removing, taken exactly once.
    pub(crate) fn take_confirmed_analysis(&mut self) -> Option<crate::product::AnalysisInstanceId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::Analysis(id) => Some(id),
            _ => None,
        })
    }

    /// The design variable the reader confirmed removing from `plan`.
    pub(crate) fn take_confirmed_variable(
        &mut self,
        plan: crate::product::SimulationPlanId,
    ) -> Option<crate::product::DesignVariableId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::Variable { plan: staged, id } if staged == plan => Some(id),
            _ => None,
        })
    }

    /// The saved output the reader confirmed removing from `plan`.
    pub(crate) fn take_confirmed_output(
        &mut self,
        plan: crate::product::SimulationPlanId,
    ) -> Option<crate::product::SavedOutputId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::Output { plan: staged, id } if staged == plan => Some(id),
            _ => None,
        })
    }

    /// The capture group the reader confirmed removing from `plan`.
    pub(crate) fn take_confirmed_capture_group(
        &mut self,
        plan: crate::product::SimulationPlanId,
    ) -> Option<crate::product::CaptureGroupId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::CaptureGroup { plan: staged, id } if staged == plan => Some(id),
            _ => None,
        })
    }

    /// A confirmed answer of one kind, taken exactly once.
    ///
    /// The review is only closed when `extract` claims it. A registry that
    /// asked for a kind it did not stage must leave the answer standing for
    /// the registry that did, or one page reading the review would silently
    /// discard another page's confirmed removal.
    fn take_confirmed<T>(&mut self, extract: impl Fn(PlanRemovalTarget) -> Option<T>) -> Option<T> {
        let taken = self.target.filter(|_| self.confirmed).and_then(extract)?;
        self.close();
        Some(taken)
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Why one analysis cannot be removed at all, and what has to move first.
///
/// This is not a review. A review asks a question the reader's answer decides;
/// this states a decision the plan has already made. The plan's removal
/// transaction refuses an instance any other instance is still bound to, and it
/// refuses it on exactly the predicate the surface can evaluate before opening
/// anything — so staging a destructive confirmation for that case asked the
/// reader to authorise something that could only ever end in a refusal notice.
///
/// The blockers are carried as `(id, name)` because both are needed and neither
/// substitutes for the other: the name is what the reader knows the analysis
/// by, and the id is what the hop to it needs.
#[derive(Debug, Clone, Default)]
pub(crate) struct PlanRemovalRefusal {
    pub(crate) analysis: Option<crate::product::AnalysisInstanceId>,
    /// The analysis the reader asked to remove, by the name it shows under.
    pub(crate) label: String,
    /// Every analysis bound to it, in plan order.
    pub(crate) blockers: Vec<(crate::product::AnalysisInstanceId, String)>,
    /// Set by the dialog when the reader asks to be shown the first blocker.
    /// The surface that owns the plan performs the hop on its next frame, so
    /// the modal never navigates another surface itself.
    pub(crate) reveal: bool,
}

impl PlanRemovalRefusal {
    pub(crate) fn open(
        &mut self,
        analysis: crate::product::AnalysisInstanceId,
        label: String,
        blockers: Vec<(crate::product::AnalysisInstanceId, String)>,
    ) {
        *self = Self {
            analysis: Some(analysis),
            label,
            blockers,
            reveal: false,
        };
    }

    /// The blocker the reader asked to be shown, taken exactly once.
    ///
    /// Taking it closes the refusal: the hop leaves this analysis behind, so
    /// leaving the notice standing over the page it lands on would state a
    /// refusal about something the reader is no longer looking at.
    pub(crate) fn take_reveal_target(&mut self) -> Option<crate::product::AnalysisInstanceId> {
        if !self.reveal {
            return None;
        }
        let target = self.blockers.first().map(|(id, _)| *id);
        self.close();
        target
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// What the hop to the blocking analysis is called.
///
/// The same words the advanced-options panel uses for the same hop, and the
/// studio pins the two together so a reword on one surface cannot leave the
/// other calling the same destination something else.
pub(in crate::workbench) const REVEAL_BLOCKER: &str = "Open in Analyses";

impl PlanRemovalTarget {
    /// The dialog's own title, which names the kind being removed.
    const fn title(self) -> &'static str {
        match self {
            Self::Analysis(_) => "Remove analysis",
            Self::Variable { .. } => "Remove design variable",
            Self::Output { .. } => "Remove saved output",
            Self::CaptureGroup { .. } => "Remove capture group",
        }
    }

    /// The key the first fact row names the record under.
    const fn subject(self) -> &'static str {
        match self {
            Self::Analysis(_) => "Analysis",
            Self::Variable { .. } => "Variable",
            Self::Output { .. } => "Saved output",
            Self::CaptureGroup { .. } => "Capture group",
        }
    }

    /// What removal does, stated before what it costs.
    ///
    /// Each of these says the same two things in the terms of its own
    /// registry: what leaves the plan, and what does not leave with it. A
    /// destructive review whose description named only the destruction would
    /// be read as deleting the evidence too.
    const fn description(self) -> &'static str {
        match self {
            Self::Analysis(_) => {
                "Remove this analysis from the plan. Retained datasets are not deleted, but they \
                 stop being attributable to a configured analysis."
            }
            Self::Variable { .. } => {
                "Remove this variable from the plan. Nothing already measured changes, but every \
                 expression that resolved through this name stops resolving."
            }
            Self::Output { .. } => {
                "Remove this output from the plan. Datasets already captured under it are not \
                 deleted, but no further run is asked to capture it."
            }
            Self::CaptureGroup { .. } => {
                "Remove this capture group from the plan. The outputs it held are not removed \
                 with it, but the overrides it applied to them stop applying."
            }
        }
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_plan_removal_review_dialog(&mut self, ctx: &Context) {
        let review = &self.state.dialogs.plan_removal_review;
        let Some(target) = review.target else {
            return;
        };

        let label = review.label.clone();
        let consequences = review.consequences.clone();

        let choice = Dialog::new(target.title(), "Remove")
            .description(target.description())
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .destructive()
            .hint("Plan mutation · not undoable from schematic history")
            .show(ctx, |ui| {
                kv_row(ui, target.subject(), &label);
                for consequence in &consequences {
                    kv_row(ui, &consequence.fact, &consequence.value);
                }

                ui.add_space(8.0);
                let t = Tokens::get(ui.ctx());
                // Warnings first, then the asides. What the reader has to
                // repair is what they are being stopped for; what survives
                // removal is said afterwards, so it cannot be mistaken for the
                // reason the review opened.
                let notes = |wanted: PlanRemovalTone| {
                    consequences
                        .iter()
                        .filter_map(|consequence| consequence.note.as_ref())
                        .filter(move |(tone, _)| *tone == wanted)
                        .map(|(_, note)| note.as_str())
                };
                for note in notes(PlanRemovalTone::Warn) {
                    ui.label(
                        egui::RichText::new(note)
                            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.warn),
                    );
                    ui.add_space(6.0);
                }
                for note in notes(PlanRemovalTone::Aside) {
                    ui.label(
                        egui::RichText::new(note)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => {
                self.state.dialogs.plan_removal_review.confirmed = true;
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.plan_removal_review.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

impl RSpiceApp {
    /// State a removal the plan will not perform, on the surface the review
    /// would have used.
    ///
    /// Not a review, and deliberately not shaped like one. The removal
    /// transaction refuses an analysis any other instance is still bound to,
    /// so a destructive confirmation for that case was a question whose only
    /// answer was a refusal notice a frame later — the reader authorised the
    /// removal, nothing was removed, and what they were told afterwards named
    /// the blocking analyses by instance id. There is no primary action that
    /// destroys anything here: the only thing to do about a blocked removal is
    /// to go and look at what is blocking it.
    pub(in crate::workbench) fn process_plan_removal_refusal_dialog(&mut self, ctx: &Context) {
        let refusal = &self.state.dialogs.plan_removal_refusal;
        if refusal.analysis.is_none() {
            return;
        }
        let label = refusal.label.clone();
        let blockers: Vec<String> = refusal
            .blockers
            .iter()
            .map(|(_, name)| name.clone())
            .collect();
        if blockers.is_empty() {
            // A refusal with nothing to name is not a refusal anyone can act
            // on. Nothing opens one, and standing one open would be a modal
            // whose only statement is that something unnamed is in the way.
            self.state.dialogs.plan_removal_refusal.close();
            return;
        }

        let choice = Dialog::new("Cannot remove analysis", REVEAL_BLOCKER)
            .description(
                "This analysis is a prerequisite of another one. Removing it is refused by the \
                 plan, so nothing has been staged and nothing has changed.",
            )
            .size(DialogSize::Transaction)
            .ghost("Close")
            .hint("The plan is unchanged · no confirmation is being asked for")
            .show(ctx, |ui| {
                kv_row(ui, "Analysis", &label);
                kv_row(ui, "Bound to it", &blockers.join(", "));

                ui.add_space(8.0);
                let t = Tokens::get(ui.ctx());
                ui.label(
                    egui::RichText::new(format!(
                        "Rebind or remove {} first. An analysis the plan still resolves a \
                         prerequisite through cannot be taken out from under it: what is bound \
                         to it would be left naming a record that is gone.",
                        blockers.join(", ")
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.warn),
                );
            });

        match choice {
            // The hop is recorded, not taken. The page that owns the plan
            // performs it on its next frame, exactly as a confirmed removal is
            // applied — this module renders and records, and navigates nothing
            // itself.
            DialogChoice::Primary => {
                self.state.dialogs.plan_removal_refusal.reveal = true;
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.plan_removal_refusal.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}
