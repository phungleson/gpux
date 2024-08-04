// Copyright 2024 Phung Le Son.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://github.com/phungleson/gpux/blob/main/LICENSE-APACHE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::{
    div, px, rgb, size, App, Bounds, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};
use gpux_nes_css::logos::{
    Discord, Facebook, Github, Gmail, Google, Instagram, Linkedin, Medium, Reddit, Twitch, Twitter,
    Whatsapp, Youtube,
};

struct Main {}

impl Render for Main {
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
            .child(Discord::new("discord"))
    }
}

fn main() {
    let app = App::new();

    app.run(move |cx| {
        let bounds = Bounds::centered(None, size(px(400.0), px(200.0)), cx);
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
