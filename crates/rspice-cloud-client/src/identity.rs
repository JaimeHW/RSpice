use rspice_cloud_contract::{API_VERSION, BuildInfo, CurrentPrincipal, SERVICE_NAME};

use crate::validation::parse_timestamp_text;

const MAX_VERSION_BYTES: usize = 64;
const MAX_SOURCE_SHA_BYTES: usize = 128;
const MAX_EMAIL_BYTES: usize = 320;
const MAX_DISPLAY_NAME_BYTES: usize = 256;

pub(crate) fn valid_build_info(build: &BuildInfo) -> bool {
    build.service == SERVICE_NAME
        && build.api_version == API_VERSION
        && valid_build_identifier(&build.version, MAX_VERSION_BYTES)
        && valid_build_identifier(&build.source_sha, MAX_SOURCE_SHA_BYTES)
}

pub(crate) fn valid_current_principal(principal: &CurrentPrincipal) -> bool {
    if principal.id.is_nil()
        || !principal.email.as_deref().is_none_or(valid_profile_email)
        || !principal
            .display_name
            .as_deref()
            .is_none_or(|value| valid_profile_text(value, MAX_DISPLAY_NAME_BYTES))
    {
        return false;
    }

    let Some(created_at) = parse_timestamp_text(&principal.created_at) else {
        return false;
    };
    parse_timestamp_text(&principal.updated_at).is_some_and(|updated_at| updated_at >= created_at)
}

fn valid_build_identifier(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_bytes
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'+'))
}

fn valid_profile_email(value: &str) -> bool {
    if !valid_profile_text(value, MAX_EMAIL_BYTES)
        || value.chars().any(char::is_whitespace)
        || value.matches('@').count() != 1
    {
        return false;
    }
    value
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && !domain.is_empty())
}

fn valid_profile_text(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_bytes
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use rspice_cloud_contract::Uuid;

    use super::*;

    #[test]
    fn build_identity_is_exact_bounded_and_service_scoped() {
        let mut build = BuildInfo {
            service: SERVICE_NAME.to_owned(),
            version: "0.1.0".to_owned(),
            source_sha: "0123456789abcdef".to_owned(),
            api_version: API_VERSION.to_owned(),
        };
        assert!(valid_build_info(&build));
        build.service = "different-service".to_owned();
        assert!(!valid_build_info(&build));
        build.service = SERVICE_NAME.to_owned();
        build.source_sha = "unsafe source".to_owned();
        assert!(!valid_build_info(&build));
    }

    #[test]
    fn principal_profiles_are_nonnil_clean_and_chronological() {
        let mut principal = CurrentPrincipal {
            id: Uuid::from_u128(1),
            email: Some("engineer@rspice.app".to_owned()),
            display_name: Some("Precision Engineer".to_owned()),
            created_at: "2026-07-19T00:00:00Z".to_owned(),
            updated_at: "2026-07-19T00:00:01Z".to_owned(),
        };
        assert!(valid_current_principal(&principal));
        principal.updated_at = "2026-07-18T00:00:00Z".to_owned();
        assert!(!valid_current_principal(&principal));
        principal.updated_at = "2026-07-19T00:00:01Z".to_owned();
        principal.email = Some("not-an-email".to_owned());
        assert!(!valid_current_principal(&principal));
    }
}
