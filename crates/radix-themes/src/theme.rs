// Copyright 2024 Phung Le Son.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://github.com/phungleson/gpux/blob/main/LICENSE-APACHE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::{FontWeight, Global, Hsla, Pixels, px, rems, Rems, rgba, SharedString};
use once_cell::sync::Lazy;

use gpux_css::color::{hsla, white};
use gpux_radix_colors::color_scale::ColorScale;
use gpux_radix_colors::color_scales::{color_scales, ColorScales};
use gpux_theme::theme_mode::ThemeMode;

use crate::theme::nine_scale::NineScale;
use crate::theme::six_scale::SixScale;

mod nine_scale;
mod six_scale;

static COLOR_SCALES: Lazy<ColorScales> = Lazy::new(color_scales);

#[derive(Default, Copy, Clone)]
pub enum AccentColor {
    Gray,
    Gold,
    Bronze,
    Brown,
    Yellow,
    Amber,
    Orange,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    #[default]
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Lime,
    Mint,
    Sky,
}

impl AccentColor {
    pub fn theme_color(&self, theme_mode: ThemeMode) -> ThemeColor {
        match self {
            AccentColor::Indigo => ThemeColor::indigo(theme_mode),
            AccentColor::Cyan => ThemeColor::cyan(theme_mode),
            AccentColor::Orange => ThemeColor::orange(theme_mode),
            AccentColor::Crimson => ThemeColor::crimson(theme_mode),
            _ => ThemeColor::indigo(theme_mode),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub enum GrayColor {
    Gray,
    Mauve,
    #[default]
    Slate,
    Sage,
    Olive,
    Sand,
}

#[derive(Default)]
pub struct ThemeColor {
    pub solid: ColorScale,
    pub transparent: ColorScale,

    // Functional colors
    pub indicator: Hsla,
    pub contrast: Hsla,
    pub surface: Hsla,
    pub track: Hsla,
}

impl ThemeColor {
    pub fn slate(theme_mode: ThemeMode) -> Self {
        let slate = COLOR_SCALES.slate.color_scale(theme_mode);
        let slate_alpha = COLOR_SCALES.slate.color_scale_alpha(theme_mode);

        Self {
            solid: slate.clone(),
            transparent: slate_alpha.clone(),
            //
            contrast: white(),
            surface: hsla(222., 100., 98., 0.80),
            indicator: slate.step_9(),
            track: slate.step_9(),
        }
    }

    pub fn indigo(theme_mode: ThemeMode) -> Self {
        let indigo = COLOR_SCALES.indigo.color_scale(theme_mode);
        let indigo_alpha = COLOR_SCALES.indigo.color_scale_alpha(theme_mode);

        Self {
            solid: indigo.clone(),
            transparent: indigo_alpha.clone(),
            //
            contrast: white(),
            surface: hsla(222., 100., 98., 0.80),
            indicator: indigo.step_9(),
            track: indigo.step_9(),
        }
    }

    pub fn cyan(theme_mode: ThemeMode) -> Self {
        let cyan = COLOR_SCALES.cyan.color_scale(theme_mode);
        let cyan_alpha = COLOR_SCALES.cyan.color_scale_alpha(theme_mode);

        Self {
            solid: cyan.clone(),
            transparent: cyan_alpha.clone(),
            //
            contrast: white(),
            surface: hsla(222., 100., 98., 0.80),
            indicator: cyan.step_9(),
            track: cyan.step_9(),
        }
    }

    pub fn orange(theme_mode: ThemeMode) -> Self {
        let orange = COLOR_SCALES.orange.color_scale(theme_mode);
        let orange_alpha = COLOR_SCALES.orange.color_scale_alpha(theme_mode);

        Self {
            solid: orange.clone(),
            transparent: orange_alpha.clone(),
            //
            contrast: white(),
            surface: hsla(222., 100., 98., 0.80),
            indicator: orange.step_9(),
            track: orange.step_9(),
        }
    }

    pub fn crimson(theme_mode: ThemeMode) -> Self {
        let crimson = COLOR_SCALES.crimson.color_scale(theme_mode);
        let crimson_alpha = COLOR_SCALES.crimson.color_scale_alpha(theme_mode);

        Self {
            solid: crimson.clone(),
            transparent: crimson_alpha.clone(),
            //
            contrast: white(),
            surface: hsla(222., 100., 98., 0.80),
            indicator: crimson.step_9(),
            track: crimson.step_9(),
        }
    }
}

#[derive(Default)]
pub struct Theme {
    // pub font_size: f32,
    pub(crate) font_family: SharedString,
    pub(crate) theme_mode: ThemeMode,
    // Accent theme colors
    indigo: ThemeColor,
    cyan: ThemeColor,
    orange: ThemeColor,
    crimson: ThemeColor,
    // Gray theme colors
    slate: ThemeColor,

    pub(crate) font_size: NineScale<Pixels>,
    pub(crate) space: NineScale<Pixels>,
    pub(crate) radius: SixScale<Pixels>,
    pub(crate) line_height: NineScale<Pixels>,
    pub(crate) letter_spacing: NineScale<Rems>,
    pub(crate) font_weight_light: FontWeight,
    pub(crate) font_weight_regular: FontWeight,
    pub(crate) font_weight_medium: FontWeight,
    pub(crate) font_weight_bold: FontWeight,
    //
    gray_color: GrayColor,
    accent_color: AccentColor,
    pub(crate) color_background: Hsla,
    pub(crate) color_overlay: Hsla,
    pub(crate) color_panel_solid: Hsla,
    pub(crate) color_panel_translucent: Hsla,
    pub(crate) color_surface: Hsla,
    pub(crate) color_transparent: Hsla,
}

impl Theme {
    pub fn new(theme_mode: ThemeMode, accent_color: AccentColor, gray_color: GrayColor) -> Theme {
        let black = &COLOR_SCALES.black;
        let scaling = 1.;

        Theme {
            font_family: if cfg!(target_os = "macos") {
                ".SystemUIFont".into()
            } else if cfg!(target_os = "windows") {
                "Segoe UI".into()
            } else {
                "FreeMono".into()
            },
            space: NineScale([
                px(4.) * scaling,
                px(8.) * scaling,
                px(12.) * scaling,
                px(16.) * scaling,
                px(24.) * scaling,
                px(32.) * scaling,
                px(40.) * scaling,
                px(48.) * scaling,
                px(64.) * scaling,
            ]),
            radius: SixScale([
                px(3.) * scaling,
                px(4.) * scaling,
                px(6.) * scaling,
                px(8.) * scaling,
                px(12.) * scaling,
                px(16.) * scaling,
            ]),
            font_size: NineScale([
                px(12.) * scaling,
                px(14.) * scaling,
                px(16.) * scaling,
                px(18.) * scaling,
                px(20.) * scaling,
                px(24.) * scaling,
                px(28.) * scaling,
                px(35.) * scaling,
                px(60.) * scaling,
            ]),
            line_height: NineScale([
                px(16.) * scaling,
                px(20.) * scaling,
                px(24.) * scaling,
                px(26.) * scaling,
                px(28.) * scaling,
                px(30.) * scaling,
                px(36.) * scaling,
                px(40.) * scaling,
                px(60.) * scaling,
            ]),
            letter_spacing: NineScale([
                rems(0.0025) * scaling,
                rems(0.) * scaling,
                rems(0.) * scaling,
                rems(-0.0025) * scaling,
                rems(-0.005) * scaling,
                rems(-0.00625) * scaling,
                rems(-0.0075) * scaling,
                rems(-0.01) * scaling,
                rems(-0.025) * scaling,
            ]),
            font_weight_light: FontWeight::LIGHT,
            font_weight_regular: FontWeight::NORMAL,
            font_weight_medium: FontWeight::MEDIUM,
            font_weight_bold: FontWeight::BOLD,
            theme_mode,
            gray_color,
            accent_color,
            // Accent colors
            indigo: ThemeColor::indigo(theme_mode),
            cyan: ThemeColor::cyan(theme_mode),
            orange: ThemeColor::orange(theme_mode),
            crimson: ThemeColor::crimson(theme_mode),
            // Gray colors
            slate: ThemeColor::slate(theme_mode),
            //
            color_background: white(),
            color_overlay: black.light_alpha().step_6(),
            color_panel_solid: white(),
            color_panel_translucent: rgba(0xffffffb3).into(),
            color_surface: rgba(0xffffffd9).into(),
            color_transparent: rgba(0x00000000).into(),
            ..Default::default()
        }
    }

    pub fn accent(&self, color: Option<AccentColor>) -> &ThemeColor {
        let accent_color = color.unwrap_or(self.accent_color);

        match accent_color {
            AccentColor::Indigo => &self.indigo,
            AccentColor::Cyan => &self.cyan,
            AccentColor::Orange => &self.orange,
            AccentColor::Crimson => &self.crimson,
            _ => &self.accent(Some(AccentColor::default())),
        }
    }

    pub fn gray(&self) -> &ThemeColor {
        match &self.gray_color {
            &GrayColor::Slate => &self.slate,
            _ => &self.slate,
        }
    }
}

impl Global for Theme {}
