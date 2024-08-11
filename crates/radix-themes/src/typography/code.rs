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

use gpui::{
    AnyElement, div, Div, IntoElement, ParentElement, Pixels, px, rems, RenderOnce, Styled,
    WindowContext,
};
use gpui::prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::theme::{AccentColor, Theme};
use crate::typography::{Size, TypographyStyle, Weight, Wrap};

#[derive(Default)]
pub enum CodeVariant {
    #[default]
    Solid,
    Soft,
    Outline,
    Ghost,
}

#[derive(IntoElement, Default)]
pub struct Code {
    children: SmallVec<[AnyElement; 2]>,
    variant: CodeVariant,
    style: TypographyStyle,
}

impl Code {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn size(mut self, size: Size) -> Self {
        self.style.size = size;
        self
    }

    pub fn weight(mut self, weight: Weight) -> Self {
        self.style.weight = weight;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.style.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.style.high_contrast = high_contrast;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.style.wrap = wrap;
        self
    }

    pub fn variant(mut self, variant: CodeVariant) -> Self {
        self.variant = variant;
        self
    }

    fn map_text_color<'a>(&'a self, theme: &'a Theme) -> impl Fn(Div) -> Div + 'a {
        |div| match self.variant {
            CodeVariant::Solid => div
                .bg(theme.accent(self.style.color).transparent.step_9())
                .text_color(theme.accent(self.style.color).contrast),
            CodeVariant::Soft => div
                .bg(theme.accent(self.style.color).transparent.step_3())
                .text_color(theme.accent(self.style.color).transparent.step_11()),
            CodeVariant::Outline => {
                div.text_color(theme.accent(self.style.color).transparent.step_11())
            }
            CodeVariant::Ghost => div,
        }
    }

    fn map_variant<'a>(&'a self, theme: &'a Theme) -> impl Fn(Div) -> Div + '_ {
        |div| match self.variant {
            CodeVariant::Solid => div
                .bg(theme.accent(self.style.color).transparent.step_9())
                .text_color(theme.accent(self.style.color).contrast),
            CodeVariant::Soft => div
                .bg(theme.accent(self.style.color).transparent.step_3())
                .text_color(theme.accent(self.style.color).transparent.step_11()),
            CodeVariant::Outline => {
                div.text_color(theme.accent(self.style.color).transparent.step_11())
            }
            CodeVariant::Ghost => div,
        }
    }

    fn border_radius(&self, rem_size: Pixels) -> Pixels {
        px(0.5) + rems(0.2).to_pixels(rem_size)
    }
}

impl ParentElement for Code {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Code {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .font_weight(self.style.font_weight(theme))
            .rounded(self.border_radius(cx.rem_size()))
            .px(rems(0.25))
            .py(rems(0.1))
            .map(self.style.map_size(theme))
            .map(self.style.map_size(theme))
            .map(self.map_variant(theme))
            .map(self.style.map_whitespace())
            .children(self.children)
    }
}

pub fn code() -> Code {
    Code::new()
}
