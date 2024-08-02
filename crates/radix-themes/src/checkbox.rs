use std::fmt;

use gpui::{div, Div, InteractiveElement, Pixels, px, rems, Rems, StatefulInteractiveElement, Styled, Svg};
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

pub enum CheckboxSize {
    One,
    Two,
    Three,
}

pub enum CheckboxVariant {
    Classic,
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
            checked: Selection::Unselected,
            disabled: false,
            label: None,
            on_click: None,
            size: CheckboxSize::Two,
            color: None,
            high_contrast: false,
            variant: CheckboxVariant::Soft,
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

    fn render_size(&self) -> Rems {
        match self.size {
            CheckboxSize::One => rems(1.) * 0.875,
            CheckboxSize::Two => rems(1.),
            CheckboxSize::Three => rems(1.) * 1.25,
        }
    }

    fn render_border_radius(&self) -> Pixels {
        match self.size {
            CheckboxSize::One => px(3.) * 0.875,
            CheckboxSize::Two => px(3.),
            CheckboxSize::Three => px(3.) * 1.25,
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

    fn render_letter_spacing(&self, theme: &Theme) -> Rems {
        match self.size {
            CheckboxSize::One => theme.letter_spacing.step_2(),
            CheckboxSize::Two => theme.letter_spacing.step_3(),
            CheckboxSize::Three => theme.letter_spacing.step_4(),
        }
    }

    fn render_check_div_variant(&self, theme: &Theme, div: Div) -> Div {
        div
            .map(|this| match self.checked {
                Selection::Unselected => this
                    .bg(match self.variant {
                        CheckboxVariant::Soft => theme.accent(self.color).surface,
                        CheckboxVariant::Classic => theme.accent(self.color).surface,
                        CheckboxVariant::Surface => theme.accent(self.color).transparent.step_5(),
                    })
                    // TODO: use box-shadow
                    .border_1()
                    .border_color(
                        match self.variant {
                            CheckboxVariant::Soft => theme.gray().transparent.step_7(),
                            // TODO: add shadow
                            CheckboxVariant::Classic => theme.gray().transparent.step_7(),
                            // Uses transparent white as border theme.accent(self.color).transparent.step_5() looks different from bg
                            CheckboxVariant::Surface => transparent_white(),
                        }
                    )
                ,
                _ => this.bg(match self.variant {
                    CheckboxVariant::Soft =>
                        if self.high_contrast { theme.accent(self.color).solid.step_12() } else { theme.accent(self.color).indicator }
                    CheckboxVariant::Classic =>
                        if self.high_contrast { theme.accent(self.color).solid.step_12() } else { theme.accent(self.color).indicator },
                    CheckboxVariant::Surface =>
                        // TODO: check docs for high contrast surface
                        if self.high_contrast { theme.accent(self.color).transparent.step_5() } else { theme.accent(self.color).transparent.step_5() },
                }),
            })
            .map(|this| match self.disabled {
                true => this.bg(theme.gray().transparent.step_3()).border_1().border_color(theme.gray().transparent.step_6()),
                _ => this,
            })
    }

    fn render_svg_variant(&self, theme: &Theme, svg: Svg) -> Svg {
        svg
            .map(|this| match self.disabled {
                true => this.text_color(theme.gray().transparent.step_8()),
                _ => this.text_color(
                    match self.variant {
                        CheckboxVariant::Soft => theme.accent(self.color).contrast,
                        CheckboxVariant::Classic => theme.accent(self.color).contrast,
                        CheckboxVariant::Surface => theme.accent(self.color).transparent.step_11(),
                    }
                ),
            })
            .map(|this| match self.checked {
                Selection::Selected => this.path(CheckboxIcon::Check.path()),
                _ => this,
            })
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

        let svg = svg().size(rems(0.625));
        let check_div = div()
            .flex()
            .justify_center()
            .items_center()
            .rounded(self.render_border_radius())
            .size(self.render_size())
            .child(self.render_svg_variant(theme, svg));

        div()
            .stack_h()
            .gap_2()
            .child(self.render_check_div_variant(theme, check_div))
            .map(|this| match self.label.as_ref() {
                Some(label) => this
                    .text_size(self.render_font_size(theme))
                    .line_height(self.render_line_height(theme))
                    // TODO: letter spacing
                    .text_color(theme.gray().solid.step_12())
                    .child(label.to_string()),
                _ => this
            })
            .id(self.id)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |_, cx| on_click(&self.checked.inverse(), cx)),
            )
    }
}

