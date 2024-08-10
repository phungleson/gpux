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

use gpui::{AnyElement, div, IntoElement, ParentElement, RenderOnce, Styled, WindowContext};
use gpui::prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::theme::{AccentColor, Theme};
use crate::typography::{Size, TypographyStyle, Weight, Wrap};

#[derive(IntoElement, Default)]
pub struct Heading {
    children: SmallVec<[AnyElement; 2]>,
    style: TypographyStyle,
}

impl Heading {
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

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.style.wrap = wrap;
        self
    }
}

impl ParentElement for Heading {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Heading {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_size(self.style.font_size(theme))
            .font_weight(self.style.font_weight(theme))
            .map(self.style.map_text_color(theme))
            .map(self.style.map_whitespace())
            .children(self.children)
    }
}

pub fn heading() -> Heading {
    Heading::new()
}
