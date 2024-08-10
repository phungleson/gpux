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

use std::fmt;

use gpui::{div, InteractiveElement, Pixels, px, rems, StatefulInteractiveElement, Styled};
use gpui::{
    ElementId, IntoElement, ParentElement, prelude::FluentBuilder as _, RenderOnce, SharedString,
    svg, WindowContext,
};

use gpux_css::color::transparent_white;
use gpux_css::stack_ext::StackExt;
use gpux_interactivity::disableable::Disableable;
use gpux_interactivity::selectable::Selectable;
use gpux_interactivity::selection::Selection;

use crate::icon::Icon;
use crate::theme::{AccentColor, Theme};

#[derive(Debug)]
pub enum CheckboxIcon {
    Check,
}

impl fmt::Display for CheckboxIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl CheckboxIcon {
    pub fn path(self) -> SharedString {
        SharedString::from(format!("icons/checkbox/{}.svg", self))
    }
}

impl RenderOnce for CheckboxIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        Icon::new(self.path())
    }
}

type OnClick = Box<dyn Fn(&Selection, &mut WindowContext) + 'static>;

#[derive(Default)]
pub enum CheckboxSize {
    One,
    #[default]
    Two,
    Three,
}

#[derive(Default)]
pub enum CheckboxVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: Selection,
    disabled: bool,
    label: Option<SharedString>,
    size: CheckboxSize,
    variant: CheckboxVariant,
    color: Option<AccentColor>,
    high_contrast: bool,
    on_click: Option<OnClick>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: Default::default(),
            color: Default::default(),
            high_contrast: Default::default(),
            variant: Default::default(),
            checked: Default::default(),
            disabled: Default::default(),
            label: Default::default(),
            on_click: Default::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn checked(mut self, checked: Selection) -> Self {
        self.checked = checked;
        self
    }

    pub fn on_click(mut self, on_click: impl Fn(&Selection, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
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

    pub fn variant(mut self, variant: CheckboxVariant) -> Self {
        self.variant = variant;
        self
    }

    fn render_size(&self, theme: &Theme) -> Pixels {
        match self.size {
            CheckboxSize::One => theme.space.step_4() * 0.875,
            CheckboxSize::Two => theme.space.step_4(),
            CheckboxSize::Three => theme.space.step_4() * 1.25,
        }
    }

    fn render_border_radius(&self, theme: &Theme) -> Pixels {
        match self.size {
            CheckboxSize::One => theme.radius.step_1() * 0.875,
            CheckboxSize::Two => theme.radius.step_1(),
            CheckboxSize::Three => theme.radius.step_1() * 1.25,
        }
    }

    fn render_font_size(&self, theme: &Theme) -> Pixels {
        match self.size {
            CheckboxSize::One => theme.font_size.step_2(),
            CheckboxSize::Two => theme.font_size.step_3(),
            CheckboxSize::Three => theme.font_size.step_4(),
        }
    }

    fn render_line_height(&self, theme: &Theme) -> Pixels {
        match self.size {
            CheckboxSize::One => theme.line_height.step_2(),
            CheckboxSize::Two => theme.line_height.step_3(),
            CheckboxSize::Three => theme.line_height.step_4(),
        }
    }
}

impl Disableable for Checkbox {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for Checkbox {
    fn selected(self, selected: bool) -> Self {
        self.checked(if selected {
            Selection::Selected
        } else {
            Selection::Unselected
        })
    }
}

impl RenderOnce for Checkbox {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .stack_h()
            .gap_2()
            .line_height(self.render_line_height(theme))
            .child(check(&self, theme))
            .text_size(self.render_font_size(theme))
            .text_color(theme.gray().solid.step_12())
            .map(|this| match self.label {
                Some(label) => this
                    // TODO: letter spacing
                    .child(div().mt(px(4.)).child(label)),
                _ => this,
            })
            .id(self.id)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |_, cx| on_click(&self.checked.inverse(), cx)),
            )
    }
}

fn indicator(checkbox: &Checkbox, theme: &Theme) -> impl IntoElement {
    svg()
        .size(rems(0.625))
        .map(|this| match checkbox.disabled {
            true => this.text_color(theme.gray().transparent.step_8()),
            _ => this.text_color(match checkbox.variant {
                CheckboxVariant::Soft => theme.accent(checkbox.color).contrast,
                CheckboxVariant::Classic => theme.accent(checkbox.color).contrast,
                CheckboxVariant::Surface => theme.accent(checkbox.color).transparent.step_11(),
            }),
        })
        .map(|this| match checkbox.checked {
            Selection::Selected => this.path(CheckboxIcon::Check.path()),
            _ => this,
        })
}

fn check(checkbox: &Checkbox, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .rounded(checkbox.render_border_radius(theme))
        .size(checkbox.render_size(theme))
        .child(indicator(&checkbox, theme))
        .map(|this| match checkbox.checked {
            Selection::Unselected => this
                .bg(match checkbox.variant {
                    CheckboxVariant::Soft => theme.accent(checkbox.color).surface,
                    CheckboxVariant::Classic => theme.accent(checkbox.color).surface,
                    CheckboxVariant::Surface => theme.accent(checkbox.color).transparent.step_5(),
                })
                // TODO: use box-shadow
                .border_1()
                .border_color(match checkbox.variant {
                    CheckboxVariant::Soft => theme.gray().transparent.step_7(),
                    // TODO: add shadow
                    CheckboxVariant::Classic => theme.gray().transparent.step_7(),
                    // Uses transparent white as border theme.accent(self.color).transparent.step_5() looks different from bg
                    CheckboxVariant::Surface => transparent_white(),
                }),
            _ => this.bg(match checkbox.variant {
                CheckboxVariant::Soft => {
                    if checkbox.high_contrast {
                        theme.accent(checkbox.color).solid.step_12()
                    } else {
                        theme.accent(checkbox.color).indicator
                    }
                }
                CheckboxVariant::Classic => {
                    if checkbox.high_contrast {
                        theme.accent(checkbox.color).solid.step_12()
                    } else {
                        theme.accent(checkbox.color).indicator
                    }
                }
                CheckboxVariant::Surface =>
                // TODO: check docs for high contrast surface
                {
                    if checkbox.high_contrast {
                        theme.accent(checkbox.color).transparent.step_5()
                    } else {
                        theme.accent(checkbox.color).transparent.step_5()
                    }
                }
            }),
        })
        .map(|this| match checkbox.disabled {
            true => this
                .bg(theme.gray().transparent.step_3())
                .border_1()
                .border_color(theme.gray().transparent.step_6()),
            _ => this,
        })
}
