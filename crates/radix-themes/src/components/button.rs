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
use gpui::{
    div, px, rems, CursorStyle, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    Pixels, Rems, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Svg, WindowContext,
};

use gpux_css::color::transparent_white;
use gpux_css::hsla_ext::HslaExt;
use gpux_css::stack_ext::StackExt;

use crate::theme::{AccentColor, Theme};

type OnClick = Box<dyn Fn(&mut WindowContext) + 'static>;

#[derive(Default)]
pub enum ButtonSize {
    One,
    #[default]
    Two,
    Three,
    Four,
}

#[derive(Default)]
pub enum ButtonVariant {
    Classic,
    #[default]
    Solid,
    Surface,
    Soft,
    Outline,
    Ghost,
}

#[derive(Default)]
pub enum ButtonRadius {
    None,
    #[default]
    Small,
    Medium,
    Large,
    Full,
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    high_contrast: bool,
    loading: bool,
    disabled: bool,
    radius: Option<ButtonRadius>,
    size: ButtonSize,
    label: Option<SharedString>,
    icon: Option<Svg>,
    on_click: Option<OnClick>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: Default::default(),
            color: Default::default(),
            high_contrast: Default::default(),
            loading: Default::default(),
            disabled: Default::default(),
            radius: Default::default(),
            size: Default::default(),
            label: Default::default(),
            on_click: Default::default(),
            icon: Default::default(),
        }
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn icon(mut self, icon: Svg) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn gap(&self) -> Rems {
        match self.size {
            ButtonSize::One => rems(0.25),
            ButtonSize::Two => rems(0.5),
            ButtonSize::Three | ButtonSize::Four => rems(0.75),
        }
    }

    fn px(&self) -> Rems {
        match self.size {
            ButtonSize::One => rems(0.5),
            ButtonSize::Two => rems(0.75),
            ButtonSize::Three => rems(1.),
            ButtonSize::Four => rems(1.25),
        }
    }

    fn border_radius(&self, theme: &Theme) -> Pixels {
        match self.radius.as_ref() {
            None => match self.size {
                ButtonSize::One => theme.radius.step_1(),
                ButtonSize::Two => theme.radius.step_2(),
                ButtonSize::Three => theme.radius.step_3(),
                ButtonSize::Four => theme.radius.step_4(),
            },
            Some(radius) => match radius {
                ButtonRadius::None => Pixels(0.),
                ButtonRadius::Small => theme.radius.step_2() * 0.75,
                ButtonRadius::Medium => theme.radius.step_2(),
                ButtonRadius::Large => theme.radius.step_2() * 1.5,
                ButtonRadius::Full => Pixels(9999.),
            },
        }
    }

    fn border_color(&self, theme: &Theme) -> Hsla {
        match self.variant {
            ButtonVariant::Classic => theme.gray().transparent.step_7(),
            ButtonVariant::Solid => transparent_white(),
            ButtonVariant::Soft => transparent_white(),
            ButtonVariant::Surface => theme.gray().transparent.step_7(),
            ButtonVariant::Outline => transparent_white(),
            ButtonVariant::Ghost => transparent_white(),
        }
    }

    fn height(&self, theme: &Theme) -> Pixels {
        match self.size {
            ButtonSize::One => theme.space.step_5(),
            ButtonSize::Two => theme.space.step_6(),
            ButtonSize::Three => theme.space.step_7(),
            ButtonSize::Four => theme.space.step_8(),
        }
    }

    fn font_size(&self, theme: &Theme) -> Pixels {
        match self.size {
            ButtonSize::One => theme.font_size.step_1(),
            ButtonSize::Two => theme.font_size.step_2(),
            ButtonSize::Three => theme.font_size.step_3(),
            ButtonSize::Four => theme.font_size.step_4(),
        }
    }

    fn line_height(&self, theme: &Theme) -> Pixels {
        match self.size {
            ButtonSize::One => theme.line_height.step_1(),
            ButtonSize::Two => theme.line_height.step_2(),
            ButtonSize::Three => theme.line_height.step_3(),
            ButtonSize::Four => theme.line_height.step_4(),
        }
    }

    fn padding_top(&self) -> Pixels {
        match self.size {
            ButtonSize::One => px(1.),
            ButtonSize::Two => px(2.),
            ButtonSize::Three => px(3.),
            ButtonSize::Four => px(3.),
        }
    }

    fn text_color(&self, theme: &Theme) -> Hsla {
        match self.variant {
            // TODO: classic
            ButtonVariant::Classic => theme.accent(self.color).contrast,
            ButtonVariant::Solid => theme.accent(self.color).contrast,
            ButtonVariant::Soft => theme.accent(self.color).transparent.step_11(),
            // TODO: surface, box-shadow
            ButtonVariant::Surface => theme.accent(self.color).transparent.step_11(),
            ButtonVariant::Outline => theme.accent(self.color).transparent.step_11(),
            ButtonVariant::Ghost => theme.accent(self.color).transparent.step_11(),
        }
    }

    fn icon_color(&self, theme: &Theme) -> Hsla {
        if self.disabled {
            self.disabled_text_color(theme)
        } else {
            self.text_color(theme)
        }
    }

    fn background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            // TODO: classic
            ButtonVariant::Classic => theme.accent(self.color).transparent.step_9(),
            ButtonVariant::Solid => theme.accent(self.color).transparent.step_9(),
            ButtonVariant::Soft => theme.accent(self.color).transparent.step_3(),
            // TODO: surface, box-shadow
            ButtonVariant::Surface => theme.accent(self.color).surface,
            ButtonVariant::Outline => transparent_white(),
            ButtonVariant::Ghost => transparent_white(),
        }
    }

    fn hovered_background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            // TODO: classic
            ButtonVariant::Classic => theme.accent(self.color).transparent.step_10(),
            ButtonVariant::Solid => theme.accent(self.color).transparent.step_10(),
            ButtonVariant::Soft => theme.accent(self.color).transparent.step_4(),
            // TODO: surface, box-shadow
            ButtonVariant::Surface => theme.accent(self.color).surface,
            ButtonVariant::Outline => transparent_white(),
            ButtonVariant::Ghost => theme.accent(self.color).transparent.step_2(),
        }
    }

    fn active_background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            // TODO: classic
            ButtonVariant::Classic => theme.accent(self.color).transparent.step_10(),
            ButtonVariant::Solid => theme
                .accent(self.color)
                .transparent
                .step_10()
                .brightness(0.92)
                .saturate(1.1),
            ButtonVariant::Soft => theme.accent(self.color).transparent.step_5(),
            // TODO: surface, box-shadow
            ButtonVariant::Surface => theme.accent(self.color).transparent.step_3(),
            ButtonVariant::Outline => theme.accent(self.color).transparent.step_3(),
            ButtonVariant::Ghost => theme.accent(self.color).transparent.step_2(),
        }
    }

    fn disabled_background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            // TODO: classic
            ButtonVariant::Classic
            | ButtonVariant::Solid
            | ButtonVariant::Soft
            | ButtonVariant::Surface
            | ButtonVariant::Outline => theme.gray().transparent.step_3(),
            ButtonVariant::Ghost => transparent_white(),
        }
    }

    fn disabled_text_color(&self, theme: &Theme) -> Hsla {
        theme.gray().transparent.step_8()
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let text_color = self.text_color(theme);
        let label_padding_top = self.padding_top();

        let mut element = div()
            .stack_h()
            .font_weight(theme.font_weight_medium)
            .gap(self.gap())
            .px(self.px())
            .text_size(self.font_size(theme))
            .line_height(self.line_height(theme))
            .h(self.height(theme))
            .rounded(self.border_radius(theme))
            .text_color(text_color)
            .bg(self.background(theme))
            // TODO: letter spacing
            // TODO: use box-shadow for border
            .border_1()
            .border_color(self.border_color(theme))
            .when(self.disabled || self.loading, |this| {
                this.cursor(CursorStyle::OperationNotAllowed)
                    .text_color(self.disabled_text_color(theme))
                    .bg(self.disabled_background(theme))
            })
            .when(self.loading, |this| this.px(theme.space.step_3()))
            // We need Stateful<Div> for active state
            .id(self.id.clone())
            .when(!(self.disabled || self.loading), |this| {
                this.hover(|style| style.bg(self.hovered_background(theme)))
                    .active(|style| style.bg(self.active_background(theme)))
            });

        if !self.loading {
            let icon_color = self.icon_color(theme);
            if let Some(icon) = self.icon {
                element = element.child(icon.size_3().text_color(icon_color));
            }
            if let Some(label) = self.label {
                element = element.child(div().pt(label_padding_top).child(label));
            }
        }

        element
    }
}

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
}
