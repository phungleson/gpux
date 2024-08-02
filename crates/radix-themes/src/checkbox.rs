use std::fmt;

use gpui::{div, InteractiveElement, px, rems, StatefulInteractiveElement, Styled};
use gpui::{
    ElementId, IntoElement, ParentElement, prelude::FluentBuilder as _, RenderOnce, SharedString,
    svg, WindowContext,
};

use gpux_css::stack_ext::StackExt;
use gpux_interactivity::disableable::Disableable;
use gpux_interactivity::selectable::Selectable;
use gpux_interactivity::selection::Selection;
use crate::color::Color;
use crate::icon::Icon;
use crate::theme::Theme;

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
    color: Option<Color>,
    on_click: Option<OnClick>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: Selection::Selected,
            disabled: false,
            label: None,
            on_click: None,
            size: CheckboxSize::One,
            color: None,
            variant: CheckboxVariant::Surface,
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
            .id(self.id)
            .gap_2()
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .rounded_md()
                    .size_4()
                    .map(|this| match self.checked {
                        Selection::Unselected => this.bg(theme.color_surface).border_1().border_color(theme.gray_alpha.step_9()),
                        _ => this.bg(theme.accent_indicator),
                    })
                    .map(|this| match self.disabled {
                        true => this.bg(theme.gray_alpha.step_3()),
                        _ => this,
                    })
                    .child(
                        svg()
                            .size(rems(0.625))
                            .text_color(theme.accent_contrast)
                            .map(|this| match self.checked {
                                Selection::Selected => this.path(CheckboxIcon::Check.path()),
                                _ => this,
                            }),
                    ),
            )
            .map(|this| match self.label.as_ref() {
                Some(label) => this
                    .text_sm()
                    .line_height(px(20.))
                    .text_color(theme.gray.step_12())
                    .child(label.to_string()),
                _ => this
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |_, cx| on_click(&self.checked.inverse(), cx)),
            )
    }
}
