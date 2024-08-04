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

use gpux_nes_css::characters::{Ash, Bulbasaur, Charmander, Kirby, Mario, Pokeball, Squirtle};

struct Main {}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0xffffff))
            .size_full()
            .items_center()
            .child(Pokeball::new("pokeball"))
            .child(Mario::new("mario"))
            .child(Ash::new("ash"))
            .child(Bulbasaur::new("bulbasaur"))
            .child(Charmander::new("charmander"))
            .child(Kirby::new("kirby"))
            .child(Squirtle::new("squirtle"))
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
        )
        .unwrap();
    });
}
