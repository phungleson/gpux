use gpui::{Global, Hsla, SharedString};
use gpux_css::color::hsla;

use crate::colors::Colors;

#[derive(Debug, Default)]
pub struct Theme {
    pub font_size: f32,
    pub font_family: SharedString,
    pub accent_indicator: Hsla,
    pub accent_contrast: Hsla,
    pub accent_surface: Hsla,
    pub accent_track: Hsla,
}

impl Theme {
    fn base() -> Theme {
        Theme {
            font_size: 16.0,
            font_family: if cfg!(target_os = "macos") {
                ".SystemUIFont".into()
            } else if cfg!(target_os = "windows") {
                "Segoe UI".into()
            } else {
                "FreeMono".into()
            },
            ..Default::default()
        }
    }

    pub fn indigo() -> Theme {
        let indigo = Colors::indigo();

        Theme {
            accent_contrast: Colors::white(),
            accent_surface: hsla(222., 100., 98., 0.80),
            accent_indicator: indigo.step_9,
            accent_track: indigo.step_9,
            ..Self::base()
        }
    }
}

impl Global for Theme {}
