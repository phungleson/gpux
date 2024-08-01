use gpui::{
    div, px, rgb, size, App, Bounds, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};
use gpux_nes_css::logos::{
    Facebook, Github, Gmail, Google, Instagram, Linkedin, Medium, Reddit, Twitch, Twitter,
    Whatsapp, Youtube,
};

struct Logos {}

impl Render for Logos {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .gap_2()
            .child(Facebook::new("facebook"))
            .child(Github::new("github"))
            .child(Gmail::new("gmail"))
            .child(Google::new("google"))
            .child(Instagram::new("instagram"))
            .child(Linkedin::new("linkedin"))
            .child(Medium::new("medium"))
            .child(Reddit::new("reddit"))
            .child(Twitch::new("twitch"))
            .child(Twitter::new("twitter"))
            .child(Whatsapp::new("whatsapp"))
            .child(Youtube::new("youtube"))
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
            |cx| cx.new_view(|_cx| Logos {}),
        )
        .unwrap();
    });
}
