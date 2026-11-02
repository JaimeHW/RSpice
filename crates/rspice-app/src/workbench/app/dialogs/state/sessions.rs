//! The pages the Help Center opens on.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HelpCenterPage {
    #[default]
    Help,
    Diagnostics,
    SupportBundle,
    LegalPrivacy,
}

impl HelpCenterPage {
    pub(crate) const ALL: [Self; 4] = [
        Self::Help,
        Self::Diagnostics,
        Self::SupportBundle,
        Self::LegalPrivacy,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Help => "Help center",
            Self::Diagnostics => "Diagnostics",
            Self::SupportBundle => "Support bundle",
            Self::LegalPrivacy => "Legal & privacy",
        }
    }
}
