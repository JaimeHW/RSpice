//! The pages the Help Center opens on.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HelpCenterPage {
    #[default]
    Help,
    ReleaseNotes,
    MigrationGuide,
    Diagnostics,
    SupportBundle,
    LegalPrivacy,
}

impl HelpCenterPage {
    pub(crate) const ALL: [Self; 6] = [
        Self::Help,
        Self::ReleaseNotes,
        Self::MigrationGuide,
        Self::Diagnostics,
        Self::SupportBundle,
        Self::LegalPrivacy,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Help => "Help Center",
            Self::ReleaseNotes => "Release notes",
            Self::MigrationGuide => "Migration guide",
            Self::Diagnostics => "Diagnostics",
            Self::SupportBundle => "Support bundle",
            Self::LegalPrivacy => "Legal & privacy",
        }
    }
}
