use gpui::{
    div, px, rgb, size, App, Bounds, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};
use helpers::{interactivity::Disableable, interactivity::Selection};
use radix_themes::{assets::Assets, checkbox::Checkbox, theme::Theme};

struct Main {}

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
            .child(Twitter::new("twitter"))
    }
}

fn main() {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Main {}),
        )
        .unwrap();
    });
}
