use gpui::{App, Bounds, div, FontWeight, IntoElement, ParentElement, px, Render, size, Styled, ViewContext, VisualContext, WindowBounds, WindowOptions};

use gpux_css::color::white;
use gpux_css::stack_ext::StackExt;
use gpux_radix_themes::{assets::Assets, theme::Theme};
use gpux_radix_themes::text::{text, TextSize, TextWeight, TextWrap};
use gpux_radix_themes::theme::{AccentColor, GrayColor};
use gpux_theme::theme_mode::ThemeMode;

struct Main {
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .p_4()
            .justify_center()
            .bg(white())
            .size_full()
            .child(div()
                .child("Size")
                .child(div().stack_v().gap_2()
                    .child(text().size(TextSize::One).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Two).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Three).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Four).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Five).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Six).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Seven).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Eight).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().size(TextSize::Nine).child("The quick brown fox jumps over the lazy dog.")))
            )
            .child(div()
                .child("Weight")
                .child(div().stack_v().gap_2()
                    .child(text().weight(TextWeight::Regular).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().weight(TextWeight::Medium).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().weight(TextWeight::Bold).child("The quick brown fox jumps over the lazy dog.")))
            )
            .child(div()
                .child("Wrap")
                .child(div().stack_v().gap_2()
                    .child(text().wrap(TextWrap::Nowrap).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().wrap(TextWrap::Wrap).child("The quick brown fox jumps over the lazy dog.")))
            )
            .child(div()
                .child("Color")
                .child(div().stack_v().gap_2()
                    .child(text().color(AccentColor::Indigo).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().color(AccentColor::Cyan).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().color(AccentColor::Orange).child("The quick brown fox jumps over the lazy dog."))
                    .child(text().color(AccentColor::Crimson).child("The quick brown fox jumps over the lazy dog.")))
            )
    }
}

fn main() {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        cx.set_global(Theme::new(ThemeMode::Light, AccentColor::Indigo, GrayColor::Slate));
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Main {
                })
            },
        )
        .unwrap();
    });
}
