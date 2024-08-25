use gpui::{
    div, px, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Svg, WindowContext,
};

use gpux_css::hsla_ext::HslaExt;
use gpux_css::stack_ext::StackExt;

use crate::theme::{AccentColor, Theme};

#[derive(Default)]
pub enum DropdownMenuSize {
    One,
    #[default]
    Two,
}

#[derive(Default)]
pub enum DropdownMenuVariant {
    #[default]
    Solid,
    Soft,
}

#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    disabled: bool,
    variant: DropdownMenuVariant,
    color: Option<AccentColor>,
    high_contrast: bool,
    size: DropdownMenuSize,
    label: Option<SharedString>,
    icon: Option<Svg>,
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: Default::default(),
            color: Default::default(),
            high_contrast: Default::default(),
            disabled: Default::default(),
            size: Default::default(),
            label: Default::default(),
            icon: Default::default(),
        }
    }

    pub fn size(mut self, size: DropdownMenuSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: DropdownMenuVariant) -> Self {
        self.variant = variant;
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

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<Svg>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    fn gap(&self, theme: &Theme) -> Pixels {
        match self.size {
            DropdownMenuSize::One => theme.space.step_1(),
            DropdownMenuSize::Two => theme.space.step_2(),
        }
    }

    fn px(&self, theme: &Theme) -> Pixels {
        match self.size {
            DropdownMenuSize::One => theme.space.step_2(),
            DropdownMenuSize::Two => theme.space.step_3(),
        }
    }

    fn line_height(&self, theme: &Theme) -> Pixels {
        match self.size {
            DropdownMenuSize::One => theme.line_height.step_1(),
            DropdownMenuSize::Two => theme.line_height.step_2(),
        }
    }

    fn height(&self, theme: &Theme) -> Pixels {
        match self.size {
            DropdownMenuSize::One => theme.space.step_5(),
            DropdownMenuSize::Two => theme.space.step_6(),
        }
    }

    fn font_size(&self, theme: &Theme) -> Pixels {
        match self.size {
            DropdownMenuSize::One => theme.font_size.step_1(),
            DropdownMenuSize::Two => theme.font_size.step_2(),
        }
    }

    fn text_color(&self, theme: &Theme) -> Hsla {
        match self.variant {
            DropdownMenuVariant::Solid => theme.accent(self.color).contrast,
            DropdownMenuVariant::Soft => theme.accent(self.color).transparent.step_11(),
        }
    }

    fn border_radius(&self, theme: &Theme) -> Pixels {
        match self.size {
            DropdownMenuSize::One => theme.radius.step_1(),
            DropdownMenuSize::Two => theme.radius.step_2(),
        }
    }

    fn background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            DropdownMenuVariant::Solid => theme.accent(self.color).transparent.step_9(),
            DropdownMenuVariant::Soft => theme.accent(self.color).transparent.step_3(),
        }
    }

    fn hovered_background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            DropdownMenuVariant::Solid => theme.accent(self.color).transparent.step_10(),
            DropdownMenuVariant::Soft => theme.accent(self.color).transparent.step_4(),
        }
    }

    fn active_background(&self, theme: &Theme) -> Hsla {
        match self.variant {
            DropdownMenuVariant::Solid => theme
                .accent(self.color)
                .transparent
                .step_10()
                .brightness(0.92)
                .saturate(1.1),
            DropdownMenuVariant::Soft => theme.accent(self.color).transparent.step_5(),
        }
    }

    fn disabled_background(&self, theme: &Theme) -> Hsla {
        theme.gray().transparent.step_3()
    }

    fn icon_color(&self, theme: &Theme) -> Hsla {
        if self.disabled {
            self.disabled_text_color(theme)
        } else {
            self.text_color(theme)
        }
    }

    fn padding_top(&self) -> Pixels {
        match self.size {
            DropdownMenuSize::One => px(1.),
            DropdownMenuSize::Two => px(2.),
        }
    }

    fn disabled_text_color(&self, theme: &Theme) -> Hsla {
        theme.gray().transparent.step_8()
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let label_padding_top = self.padding_top();

        let mut element = div()
            .stack_h()
            .font_weight(theme.font_weight_medium)
            .bg(self.background(theme))
            .hover(|style| style.bg(self.hovered_background(theme)))
            .gap(self.gap(theme))
            .px(self.px(theme))
            .line_height(self.line_height(theme))
            .h(self.height(theme))
            .text_size(self.font_size(theme))
            .rounded(self.border_radius(theme))
            .text_color(self.text_color(theme))
            .id(self.id.clone())
            .active(|style| style.bg(self.active_background(theme)));

        let icon_color = self.icon_color(theme);
        if let Some(label) = self.label {
            element = element.child(div().pt(label_padding_top).child(label));
        }
        if let Some(icon) = self.icon {
            element = element.child(icon.size_3().text_color(icon_color));
        }

        element
    }
}

pub fn dropdown_menu(id: impl Into<ElementId>) -> DropdownMenu {
    DropdownMenu::new(id)
}
