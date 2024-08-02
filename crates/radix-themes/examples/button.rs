use gpui::{
    App, Bounds, div, IntoElement, px, Render, rgb, size, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};

use gpux_interactivity::selection::Selection;
use gpux_radix_themes::{assets::Assets, theme::Theme};
use gpux_theme::theme_mode::ThemeMode;

struct Main {
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .border_1()
            .text_color(rgb(0xffffff))
    }
}

fn main() {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        cx.set_global(Theme::indigo(&ThemeMode::Light));
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
