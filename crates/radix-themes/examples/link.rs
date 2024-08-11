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

use gpui::{div, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};

use gpux_css::color::white;
use gpux_css::stack_ext::StackExt;
use gpux_radix_themes::theme::AccentColor;
use gpux_radix_themes::typography::{link, Size, Weight};

use crate::app::run_app;

mod app;

struct Main {}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .p_4()
            .justify_center()
            .bg(white())
            .size_full()
            .child(
                div().child("Size").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(link("one").size(Size::One).child("Sign up"))
                        .child(link("two").size(Size::Two).child("Sign up"))
                        .child(link("three").size(Size::Three).child("Sign up"))
                        .child(link("four").size(Size::Four).child("Sign up"))
                        .child(link("five").size(Size::Five).child("Sign up"))
                        .child(link("six").size(Size::Six).child("Sign up"))
                        .child(link("seven").size(Size::Seven).child("Sign up"))
                        .child(link("eight").size(Size::Eight).child("Sign up"))
                        .child(link("nine").size(Size::Nine).child("Sign up")),
                ),
            )
            .child(
                div().child("Weight").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(link("light").weight(Weight::Light).child("Sign up"))
                        .child(link("regular").weight(Weight::Regular).child("Sign up"))
                        .child(link("medium").weight(Weight::Medium).child("Sign up"))
                        .child(link("bold").weight(Weight::Bold).child("Sign up")),
                ),
            )
            .child(
                div().child("Color").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(link("indigo").color(AccentColor::Indigo).child("Sign up"))
                        .child(link("cyan").color(AccentColor::Cyan).child("Sign up"))
                        .child(link("orange").color(AccentColor::Orange).child("Sign up"))
                        .child(link("crimson").color(AccentColor::Crimson).child("Sign up")),
                ),
            )
    }
}

fn main() {
    run_app(|_cx| Main {});
}
