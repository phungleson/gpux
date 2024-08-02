use gpui::{
    App, Bounds, div, IntoElement, ParentElement, px, Render, rgb, size, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};

use gpux_nes_css::characters::Pokeball;

struct Characters {}

impl Render for Characters {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .items_center()
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
            .child(Pokeball::new("pokeball"))
    }
}

fn main() {
    let app = App::new();

    app.run(move |cx| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Characters {}),
        )
        .unwrap();
    });
}
