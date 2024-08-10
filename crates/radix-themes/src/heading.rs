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
    AnyElement, div, FontWeight, IntoElement, ParentElement, Pixels, RenderOnce, Styled,
    WindowContext,
};
use gpui::prelude::FluentBuilder;
use smallvec::SmallVec;

use crate::theme::{AccentColor, Theme};

#[derive(Copy, Clone, Default)]
pub enum HeadingSize {
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
pub enum HeadingWeight {
    Light,
    #[default]
    Regular,
    Medium,
    Bold,
}

#[derive(Copy, Clone, Default)]
pub enum HeadingWrap {
    #[default]
    Nowrap,
    Wrap,
}

#[derive(IntoElement, Default)]
pub struct Heading {
    children: SmallVec<[AnyElement; 2]>,
    size: HeadingSize,
    weight: HeadingWeight,
    wrap: HeadingWrap,
    color: Option<AccentColor>,
    high_contrast: bool,
}

impl Heading {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn size(mut self, size: HeadingSize) -> Self {
        self.size = size;
        self
    }

    pub fn weight(mut self, weight: HeadingWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn wrap(mut self, wrap: HeadingWrap) -> Self {
        self.wrap = wrap;
        self
    }

    fn render_text_size(&self, theme: &Theme) -> Pixels {
        match self.size {
            HeadingSize::One => theme.font_size.step_1(),
            HeadingSize::Two => theme.font_size.step_2(),
            HeadingSize::Three => theme.font_size.step_3(),
            HeadingSize::Four => theme.font_size.step_4(),
            HeadingSize::Five => theme.font_size.step_5(),
            HeadingSize::Six => theme.font_size.step_6(),
            HeadingSize::Seven => theme.font_size.step_7(),
            HeadingSize::Eight => theme.font_size.step_8(),
            HeadingSize::Nine => theme.font_size.step_9(),
        }
    }

    fn render_font_weight(&self, theme: &Theme) -> FontWeight {
        match self.weight {
            HeadingWeight::Light => theme.font_weight_light,
            HeadingWeight::Regular => theme.font_weight_regular,
            HeadingWeight::Medium => theme.font_weight_medium,
            HeadingWeight::Bold => theme.font_weight_bold,
        }
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
            .text_size(self.render_text_size(theme))
            .font_weight(self.render_font_weight(theme))
            .children(self.children)
            // Sets color
            .map(|this| match self.color {
                Some(accent_color) => this.text_color(
                    accent_color
                        .theme_color(theme.theme_mode)
                        .transparent
                        .step_11(),
                ),
                None => this,
            })
            .map(|this| match self.wrap {
                HeadingWrap::Nowrap => this.whitespace_nowrap(),
                HeadingWrap::Wrap => this.whitespace_normal(),
            })
    }
}

pub fn heading() -> Heading {
    Heading::new()
}
