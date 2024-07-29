use std::fmt;

use gpui::{div, rems, transparent_white, InteractiveElement, StatefulInteractiveElement, Styled};
use gpui::{
    prelude::FluentBuilder as _, svg, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, WindowContext,
};

use gpux_css::stack_ext::StackExt;
use gpux_interactivity::disableable::Disableable;
use gpux_interactivity::selectable::Selectable;
use gpux_interactivity::selection::Selection;

use crate::colors::Colors;
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

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: Selection,
    disabled: bool,
    label: Option<SharedString>,
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
                    .border_1()
                    .border_color(theme.accent_indicator)
                    .rounded_sm()
                    .shadow_lg()
                    .size_4()
                    .map(|this| match self.checked {
                        Selection::Unselected => this.bg(transparent_white()),
                        _ => this.bg(theme.accent_indicator),
                    })
                    .map(|this| {
                        if !self.disabled {
                            return this;
                        }

                        this.bg(Colors::slate_alpha().step_3)
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
            .map(|this| {
                if let Some(label) = self.label.as_ref() {
                    this.child(label.to_string())
                        .text_color(theme.accent_contrast)
                } else {
                    this
                }
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |_, cx| on_click(&self.checked.inverse(), cx)),
            )
    }
}
