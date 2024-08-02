use gpui::{Global, Hsla, rgb, rgba, SharedString};
use once_cell::sync::Lazy;

use gpux_css::color::hsla;
use gpux_css::color::white;
use gpux_radix_colors::color_scale::ColorScale;
use gpux_radix_colors::color_scales::{ColorScales, default_color_scales};
use gpux_theme::theme_mode::ThemeMode;

static COLOR_SCALES: Lazy<ColorScales> = Lazy::new(default_color_scales);

#[derive(Default)]
pub struct Theme {
    pub font_size: f32,
    pub font_family: SharedString,
    //
    pub gray: ColorScale,
    pub gray_alpha: ColorScale,
    //
    pub gray_indicator: Hsla,
    pub gray_contrast: Hsla,
    pub gray_surface: Hsla,
    pub gray_track: Hsla,
    //
    pub accent: ColorScale,
    pub accent_alpha: ColorScale,
    //
    pub accent_indicator: Hsla,
    pub accent_contrast: Hsla,
    pub accent_surface: Hsla,
    pub accent_track: Hsla,
    //
    pub color_background: Hsla,
    pub color_overlay: Hsla,
    pub color_panel_solid: Hsla,
    pub color_panel_translucent: Hsla,
    pub color_surface: Hsla,
    pub color_transparent: Hsla,
}

impl Theme {
    fn base() -> Theme {
        let black = &COLOR_SCALES.black;

        Theme {
            font_size: 16.0,
            font_family: if cfg!(target_os = "macos") {
                ".SystemUIFont".into()
            } else if cfg!(target_os = "windows") {
                "Segoe UI".into()
            } else {
                "FreeMono".into()
            },
            color_background: white(),
            color_overlay: black.light_alpha().step_6(),
            color_panel_solid: white(),
            color_panel_translucent: rgba(0xffffffb3).into(),
            color_surface: rgba(0xffffffd9).into(),
            color_transparent: rgba(0x00000000).into(),
            ..Default::default()
        }
    }

    pub fn indigo(theme_mode: &ThemeMode) -> Theme {
        let indigo = COLOR_SCALES.indigo.color_scale(theme_mode);
        let indigo_alpha = COLOR_SCALES.indigo.color_scale(theme_mode);
        let slate = COLOR_SCALES.slate.color_scale(theme_mode);
        let slate_alpha = COLOR_SCALES.slate.color_scale_alpha(theme_mode);

        Theme {
            gray: slate.clone(),
            gray_alpha: slate_alpha.clone(),
            //
            gray_contrast: white(),
            gray_surface: hsla(222., 100., 98., 0.80),
            gray_indicator: slate.step_9(),
            gray_track: slate.step_9(),

            accent: indigo.clone(),
            accent_alpha: indigo_alpha.clone(),
            //
            accent_contrast: white(),
            accent_surface: hsla(222., 100., 98., 0.80),
            accent_indicator: indigo.step_9(),
            accent_track: indigo.step_9(),
            ..Self::base()
        }
    }
}

impl Global for Theme {}
