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

/// Whom the account button names: the signed-in account's display name or
/// email, else the license holder for an offline installation; `None` when
/// neither names anyone.
pub(crate) fn account_holder(
    cloud: &CloudSessionSnapshot,
    license: Option<&LicenseInfo>,
) -> Option<String> {
    let cloud_identity = cloud.signed_in().then(|| {
        cloud.principal.as_ref().and_then(|principal| {
            principal
                .display_name
                .clone()
                .or_else(|| principal.email.clone())
        })
    });
    let license_identity = license.map(|license| license.licensed_to.trim().to_owned());
    cloud_identity
        .flatten()
        .or(license_identity)
        .filter(|name| !name.is_empty())
}

/// The user's name, from the first source that has one: the account, then
/// `license_holder`, whom the license names.
pub(crate) fn user_display_name(
    cloud: &CloudSessionSnapshot,
    license_holder: Option<&str>,
) -> String {
    let account = cloud
        .principal
        .as_ref()
        .and_then(|principal| principal.display_name.as_deref());
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
    ["USERNAME", "USER", "LOGNAME"]
        .into_iter()
        .find_map(|name| {
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
        let license = license("Ada Lovelace");
        let holder = Some(license.licensed_to.as_str());
        assert_eq!(user_display_name(&cloud, holder), "Ada Lovelace");

        cloud.principal = Some(crate::services::cloud_account::PrincipalSummary {
            display_name: Some("Grace Hopper".to_owned()),
            ..Default::default()
        });
        assert_eq!(user_display_name(&cloud, holder), "Grace Hopper");

        // A blank name is no name: the next source answers.
        cloud.principal = Some(crate::services::cloud_account::PrincipalSummary {
            display_name: Some("  ".to_owned()),
            ..Default::default()
        });
        assert_eq!(user_display_name(&cloud, holder), "Ada Lovelace");

        let unlicensed = user_display_name(&CloudSessionSnapshot::default(), None);
        assert_eq!(
            unlicensed,
            operating_system_user().unwrap_or_else(|| UNNAMED_USER.to_owned())
        );
        assert_ne!(unlicensed, "Local project editor");
    }
}
