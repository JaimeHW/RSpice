//! The one name the application knows its user by.
//!
//! Review authors, "My assignments" and the title-bar avatar all name the
//! person at this application. They read it here, in one order: the
//! signed-in cloud account's display name, then the license holder, then the
//! operating-system user. Two surfaces never call the same person two
//! different things, and none of them calls anybody "Local project editor".

use crate::services::cloud_account::CloudSessionSnapshot;
use crate::services::license::LicenseInfo;

/// What the application calls the person using it when nothing else names
/// them: no account, no license, and no operating-system user (a browser).
const UNNAMED_USER: &str = "Local user";

/// The user's name, from the first source that has one.
pub(crate) fn user_display_name(
    cloud: &CloudSessionSnapshot,
    license: Option<&LicenseInfo>,
) -> String {
    let account = cloud
        .principal
        .as_ref()
        .and_then(|principal| principal.display_name.as_deref());
    let license_holder = license.map(|info| info.licensed_to.as_str());
    [account, license_holder]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|name| !name.is_empty())
        .map(str::to_owned)
        .or_else(operating_system_user)
        .unwrap_or_else(|| UNNAMED_USER.to_owned())
}

fn operating_system_user() -> Option<String> {
    ["USERNAME", "USER", "LOGNAME"].into_iter().find_map(|name| {
        std::env::var(name)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn license(holder: &str) -> LicenseInfo {
        LicenseInfo {
            licensed_to: holder.to_owned(),
            tier: "Pro".to_owned(),
            updates_until: "2027-10-20".to_owned(),
            license_id: "identity-test".to_owned(),
            features: Vec::new(),
            updates_expired: false,
        }
    }

    #[test]
    fn the_account_name_wins_then_the_license_holder_then_the_os_user() {
        let mut cloud = CloudSessionSnapshot::default();
        let holder = license("Ada Lovelace");
        assert_eq!(user_display_name(&cloud, Some(&holder)), "Ada Lovelace");

        cloud.principal = Some(crate::services::cloud_account::PrincipalSummary {
            display_name: Some("Grace Hopper".to_owned()),
            ..Default::default()
        });
        assert_eq!(user_display_name(&cloud, Some(&holder)), "Grace Hopper");

        // A blank name is no name: the next source answers.
        cloud.principal = Some(crate::services::cloud_account::PrincipalSummary {
            display_name: Some("  ".to_owned()),
            ..Default::default()
        });
        assert_eq!(user_display_name(&cloud, Some(&holder)), "Ada Lovelace");

        let unlicensed = user_display_name(&CloudSessionSnapshot::default(), None);
        assert_eq!(
            unlicensed,
            operating_system_user().unwrap_or_else(|| UNNAMED_USER.to_owned())
        );
        assert_ne!(unlicensed, "Local project editor");
    }
}
