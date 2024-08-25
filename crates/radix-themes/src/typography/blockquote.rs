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

use gpui::prelude::FluentBuilder;
use gpui::{div, rems, AnyElement, IntoElement, ParentElement, RenderOnce, Styled, WindowContext};
use smallvec::SmallVec;

use crate::theme::{AccentColor, Theme};
use crate::typography::{Size, TypographyStyle, Weight, Wrap};

#[derive(IntoElement, Default)]
pub struct Blockquote {
    children: SmallVec<[AnyElement; 2]>,
    style: TypographyStyle,
}

impl Blockquote {
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
}

impl ParentElement for Blockquote {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Blockquote {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .font_weight(self.style.font_weight(theme))
            .map(self.style.map_size(theme))
            .map(self.style.map_text_color(theme))
            .map(self.style.map_whitespace())
            .border_l(
                theme
                    .space
                    .step_1()
                    .max(rems(0.25).to_pixels(cx.rem_size())),
            )
            .border_color(theme.accent(self.style.color).transparent.step_6())
            .pl(theme
                .space
                .step_5()
                .min(theme.space.step_3().max(rems(0.5).to_pixels(cx.rem_size()))))
            .children(self.children)
    }
}

pub fn blockquote() -> Blockquote {
    Blockquote::new()
}
