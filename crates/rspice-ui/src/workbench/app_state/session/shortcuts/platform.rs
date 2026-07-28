use crate::workbench::commands::CommandPlatform;
use crate::workbench::state::WidthClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "all host variants are exercised in tests but only one is constructed per target"
)]
enum RuntimeHost {
    Browser,
    NativeDesktop,
    NativeMobile,
}

pub(crate) fn runtime_command_platform(ctx: &egui::Context) -> CommandPlatform {
    command_platform_for_host(RuntimeHost::current(), ctx.content_rect().width())
}

const fn command_platform_for_host(host: RuntimeHost, viewport_width: f32) -> CommandPlatform {
    match host {
        // Browser-reserved shortcuts remain Browser at every responsive width.
        RuntimeHost::Browser => CommandPlatform::Browser,
        // A narrow desktop window is still a desktop host.
        RuntimeHost::NativeDesktop => CommandPlatform::Desktop,
        RuntimeHost::NativeMobile => match WidthClass::for_width(viewport_width) {
            WidthClass::Phone => CommandPlatform::Phone,
            WidthClass::Tablet | WidthClass::Desktop | WidthClass::Wide => CommandPlatform::Tablet,
        },
    }
}

impl RuntimeHost {
    const fn current() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self::Browser
        }
        #[cfg(all(
            not(target_arch = "wasm32"),
            any(target_os = "android", target_os = "ios")
        ))]
        {
            Self::NativeMobile
        }
        #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
        {
            Self::NativeDesktop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_and_responsive_authorities_are_not_conflated() {
        assert_eq!(
            command_platform_for_host(RuntimeHost::Browser, 390.0),
            CommandPlatform::Browser
        );
        assert_eq!(
            command_platform_for_host(RuntimeHost::Browser, 1_440.0),
            CommandPlatform::Browser
        );
        assert_eq!(
            command_platform_for_host(RuntimeHost::NativeDesktop, 390.0),
            CommandPlatform::Desktop
        );
        assert_eq!(
            command_platform_for_host(RuntimeHost::NativeMobile, 560.0),
            CommandPlatform::Phone
        );
        assert_eq!(
            command_platform_for_host(RuntimeHost::NativeMobile, 561.0),
            CommandPlatform::Tablet
        );
    }
}
