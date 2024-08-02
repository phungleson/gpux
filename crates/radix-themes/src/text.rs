use gpui::{AnyElement, div, Div, FontWeight, IntoElement, ParentElement, Pixels, RenderOnce, Styled, WindowContext};
use gpui::prelude::FluentBuilder;
use smallvec::SmallVec;
use gpux_theme::theme_mode::ThemeMode;
use crate::theme::{AccentColor, Theme};

#[derive(Copy, Clone, Default)]
pub enum TextSize {
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
pub enum TextWeight {
    #[default]
    Regular,
    Medium,
    Bold,
}

#[derive(Copy, Clone, Default)]
pub enum TextWrap {
    #[default]
    Nowrap,
    Wrap,
}


#[derive(IntoElement, Default)]
pub struct Text {
    children: SmallVec<[AnyElement; 2]>,
    size: TextSize,
    weight: TextWeight,
    wrap: TextWrap,
    color: Option<AccentColor>,
}

impl Text {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = wrap;
        self
    }

    fn render_text_size(&self, theme: &Theme) -> Pixels {
        match self.size {
            TextSize::One => theme.font_size.step_1(),
            TextSize::Two => theme.font_size.step_2(),
            TextSize::Three => theme.font_size.step_3(),
            TextSize::Four => theme.font_size.step_4(),
            TextSize::Five => theme.font_size.step_5(),
            TextSize::Six => theme.font_size.step_6(),
            TextSize::Seven => theme.font_size.step_7(),
            TextSize::Eight => theme.font_size.step_8(),
            TextSize::Nine => theme.font_size.step_9()
        }
    }

    fn render_font_weight(&self, theme: &Theme) -> FontWeight {
        match self.weight {
            TextWeight::Regular => theme.font_weight_regular,
            TextWeight::Medium => theme.font_weight_medium,
            TextWeight::Bold => theme.font_weight_bold,
        }
    }
}

impl ParentElement for Text {
    fn extend(&mut self, elements: impl IntoIterator<Item=AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Text {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let mut div = div()
            .text_size(self.render_text_size(theme))
            .font_weight(self.render_font_weight(theme))
            .children(self.children);

        // Sets color
        div = div.map(|this| {
            match self.color {
                Some(accent_color) => this.text_color(accent_color.theme_color(theme.theme_mode).transparent.step_11()),
                None => this,
            }
        });

        // Sets wrap
        div = div.map(|this| {
            match self.wrap {
                TextWrap::Nowrap => this.whitespace_nowrap(),
                TextWrap::Wrap => this.whitespace_normal(),
            }
        });

        div
    }
}

pub fn text() -> Text {
    Text::new()
}