use gpui::{
    App, Bounds, div, IntoElement, ParentElement, px, Render, rgb, size, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};
use gpux_nes_css::assets::Assets;

struct Main {}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0xffffff))
            .size_full()
            .items_center()
            .font_family("Press Start 2P")
            .child("Text")
    }
}

fn main() {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        Assets::init(cx).unwrap();

        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Main {}),
        ).unwrap();
    });
}
