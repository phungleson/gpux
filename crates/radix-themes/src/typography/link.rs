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
    AnyElement, div, Div, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, StyleRefinement, WindowContext,
};
use gpui::prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::theme::{AccentColor, Theme};
use crate::typography::{Size, TypographyStyle, Weight, Wrap};

#[derive(Default)]
pub enum LinkUnderline {
    #[default]
    Auto,
    Always,
    Hover,
    None,
}

#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    style: TypographyStyle,
    underline: LinkUnderline,
}

impl Link {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Link {
            id: id.into(),
            children: Default::default(),
            style: Default::default(),
            underline: Default::default(),
        }
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

    fn map_text_color<'a>(&'a self, theme: &'a Theme) -> impl Fn(Div) -> Div + '_ {
        |div| div.text_color(theme.accent(self.style.color).transparent.step_11())
    }

    fn hover<'a>(&'a self, theme: &'a Theme) -> impl Fn(StyleRefinement) -> StyleRefinement + '_ {
        |style| style
    }

    fn map_decoration<'a>(&'a self, theme: &'a Theme) -> impl Fn(Div) -> Div + '_ {
        |div| {
            div.map(|this| match self.style.size {
                Size::One | Size::Two | Size::Three | Size::Four | Size::Five | Size::Six => {
                    this.text_decoration_1()
                }
                _ => this.text_decoration_2(),
            })
            .text_decoration_color(
                theme
                    .accent(self.style.color)
                    .transparent
                    .step_5()
                    .blend(theme.gray().transparent.step_6()),
            )
        }
    }
}

impl ParentElement for Link {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Link {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .map(self.map_decoration(theme))
            .hover(self.hover(theme))
            .map(self.map_text_color(theme))
            .font_weight(self.style.font_weight(theme))
            .map(self.style.map_size(theme))
            .map(self.style.map_whitespace())
            .cursor_pointer()
            .children(self.children)
    }
}

pub fn link(id: impl Into<ElementId>) -> Link {
    Link::new(id)
}
