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

use gpui::{Div, FontWeight, Styled};

pub use blockquote::*;
pub use code::*;
pub use em::*;
pub use heading::*;
pub use kbd::*;
pub use link::*;
pub use strong::*;
pub use text::*;

use crate::theme::{AccentColor, Theme};

mod blockquote;
mod code;
mod em;
mod heading;
mod kbd;
mod link;
mod strong;
mod text;

#[derive(Default)]
pub(crate) struct TypographyStyle {
    pub(crate) size: Size,
    pub(crate) weight: Weight,
    pub(crate) wrap: Wrap,
    pub(crate) color: Option<AccentColor>,
    pub(crate) high_contrast: bool,
}

impl TypographyStyle {
    pub(crate) fn map_text_color<'a>(&'a self, theme: &'a Theme) -> impl Fn(Div) -> Div + '_ {
        |div| match self.color {
            Some(accent_color) => div.text_color(
                accent_color
                    .theme_color(theme.theme_mode)
                    .transparent
                    .step_11(),
            ),
            None => div,
        }
    }

    pub(crate) fn map_size<'a>(&'a self, theme: &'a Theme) -> impl Fn(Div) -> Div + '_ {
        |div| match self.size {
            Size::One => div
                .text_size(theme.font_size.step_1())
                .line_height(theme.line_height.step_1()),
            Size::Two => div
                .text_size(theme.font_size.step_2())
                .line_height(theme.line_height.step_2()),
            Size::Three => div
                .text_size(theme.font_size.step_3())
                .line_height(theme.line_height.step_3()),
            Size::Four => div
                .text_size(theme.font_size.step_4())
                .line_height(theme.line_height.step_4()),
            Size::Five => div
                .text_size(theme.font_size.step_5())
                .line_height(theme.line_height.step_5()),
            Size::Six => div
                .text_size(theme.font_size.step_6())
                .line_height(theme.line_height.step_6()),
            Size::Seven => div
                .text_size(theme.font_size.step_7())
                .line_height(theme.line_height.step_7()),
            Size::Eight => div
                .text_size(theme.font_size.step_8())
                .line_height(theme.line_height.step_8()),
            Size::Nine => div
                .text_size(theme.font_size.step_9())
                .line_height(theme.line_height.step_9()),
        }
    }

    pub(crate) fn font_weight(&self, theme: &Theme) -> FontWeight {
        match self.weight {
            Weight::Light => theme.font_weight_light,
            Weight::Regular => theme.font_weight_regular,
            Weight::Medium => theme.font_weight_medium,
            Weight::Bold => theme.font_weight_bold,
        }
    }

    pub(crate) fn map_whitespace(&self) -> impl Fn(Div) -> Div + '_ {
        |div| match self.wrap {
            Wrap::Nowrap => div.whitespace_nowrap(),
            Wrap::Wrap => div.whitespace_normal(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub enum Size {
    One,
    Two,
    #[default]
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Copy, Clone, Default)]
pub enum Weight {
    Light,
    #[default]
    Regular,
    Medium,
    Bold,
}

#[derive(Copy, Clone, Default)]
pub enum Wrap {
    #[default]
    Nowrap,
    Wrap,
}
