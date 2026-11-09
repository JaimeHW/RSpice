//! Whether Windows is running a contrast theme, and the colors it set.
//!
//! The system's own question and answer: `SPI_GETHIGHCONTRAST` is the supported
//! way to ask, and the theme's colors are the system colors.

use crate::ui::palette::ContrastInks;

#[cfg(windows)]
pub(crate) fn read() -> Option<ContrastInks> {
    use windows_sys::Win32::Graphics::Gdi::{
        COLOR_GRAYTEXT, COLOR_HIGHLIGHT, COLOR_HIGHLIGHTTEXT, COLOR_HOTLIGHT, COLOR_WINDOW,
        COLOR_WINDOWTEXT, GetSysColor,
    };
    use windows_sys::Win32::UI::Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTW};
    use windows_sys::Win32::UI::WindowsAndMessaging::{SPI_GETHIGHCONTRAST, SystemParametersInfoW};

    let size = std::mem::size_of::<HIGHCONTRASTW>() as u32;
    let mut state = HIGHCONTRASTW {
        cbSize: size,
        dwFlags: 0,
        lpszDefaultScheme: std::ptr::null_mut(),
    };
    // SAFETY: `state` outlives the call and `size` is its size.
    let answered =
        unsafe { SystemParametersInfoW(SPI_GETHIGHCONTRAST, size, (&raw mut state).cast(), 0) };
    if answered == 0 || state.dwFlags & HCF_HIGHCONTRASTON == 0 {
        return None;
    }
    // SAFETY: takes a color index and returns a value; nothing is borrowed.
    let system_color = |index| color_from_colorref(unsafe { GetSysColor(index) });
    Some(ContrastInks {
        window: system_color(COLOR_WINDOW),
        window_text: system_color(COLOR_WINDOWTEXT),
        highlight: system_color(COLOR_HIGHLIGHT),
        highlight_text: system_color(COLOR_HIGHLIGHTTEXT),
        link: system_color(COLOR_HOTLIGHT),
        disabled_text: system_color(COLOR_GRAYTEXT),
    })
}

/// Windows writes a color 0x00BBGGRR.
#[cfg(windows)]
fn color_from_colorref(color: u32) -> egui::Color32 {
    let [red, green, blue, _] = color.to_le_bytes();
    egui::Color32::from_rgb(red, green, blue)
}

#[cfg(not(windows))]
pub(crate) const fn read() -> Option<ContrastInks> {
    None
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn a_system_color_is_read_blue_first() {
        assert_eq!(
            color_from_colorref(0x0012_3456),
            egui::Color32::from_rgb(0x56, 0x34, 0x12)
        );
    }
}
