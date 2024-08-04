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

use crate::color_scale::ColorScale;
use gpui::SharedString;
use gpux_theme::theme_mode::ThemeMode;

/// Provides groups of [`ColorScale`]s for light and dark themes, as well as transparent versions of each scale.
pub struct ColorScaleSet {
    name: SharedString,
    light: ColorScale,
    dark: ColorScale,
    light_alpha: ColorScale,
    dark_alpha: ColorScale,
}

impl ColorScaleSet {
    pub fn new(
        name: impl Into<SharedString>,
        light: ColorScale,
        light_alpha: ColorScale,
        dark: ColorScale,
        dark_alpha: ColorScale,
    ) -> Self {
        Self {
            name: name.into(),
            light,
            light_alpha,
            dark,
            dark_alpha,
        }
    }

    pub fn name(&self) -> &SharedString {
        &self.name
    }

    pub fn color_scale(&self, theme_mode: ThemeMode) -> &ColorScale {
        match theme_mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }

    pub fn color_scale_alpha(&self, theme_mode: ThemeMode) -> &ColorScale {
        match theme_mode {
            ThemeMode::Light => &self.light_alpha,
            ThemeMode::Dark => &self.dark_alpha,
        }
    }

    pub fn light(&self) -> &ColorScale {
        &self.light
    }

    pub fn light_alpha(&self) -> &ColorScale {
        &self.light_alpha
    }

    pub fn dark(&self) -> &ColorScale {
        &self.dark
    }

    pub fn dark_alpha(&self) -> &ColorScale {
        &self.dark_alpha
    }
}
