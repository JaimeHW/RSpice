//! The pages the Help Center opens on.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HelpCenterPage {
    Diagnostics,
    SupportBundle,
    #[default]
    LegalPrivacy,
}

impl HelpCenterPage {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Diagnostics => "Diagnostics",
            Self::SupportBundle => "Support bundle",
            Self::LegalPrivacy => "Legal & privacy",
        }
    }
}
