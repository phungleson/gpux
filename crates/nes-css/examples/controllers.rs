use gpui::{
    App, Bounds, div, IntoElement, ParentElement, px, Render, rgb, size, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};

use gpux_nes_css::controllers::{Nes, NesJp, Snes, SnesJp};

struct Main {}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0xffffff))
            .size_full()
            .items_center()
            .child(Nes::new("nes"))
            .child(NesJp::new("nes-jp"))
            .child(Snes::new("snes"))
            .child(SnesJp::new("snes-jp"))
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
            |cx| cx.new_view(|_cx| Main {}),
        ).unwrap();
    });
}
