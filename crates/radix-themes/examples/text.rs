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
    div, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext,
};

use gpux_css::color::white;
use gpux_css::stack_ext::StackExt;
use gpux_radix_themes::theme::AccentColor;
use gpux_radix_themes::typography::{Size, Weight};
use gpux_radix_themes::typography::text;
use gpux_radix_themes::typography::Wrap::Wrap;

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
                        .child(
                            text()
                                .size(Size::One)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Two)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Three)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Four)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Five)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Six)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Seven)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Eight)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .size(Size::Nine)
                                .child("The quick brown fox jumps over the lazy dog."),
                        ),
                ),
            )
            .child(
                div().child("Weight").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(
                            text()
                                .weight(Weight::Regular)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .weight(Weight::Medium)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .weight(Weight::Bold)
                                .child("The quick brown fox jumps over the lazy dog."),
                        ),
                ),
            )
            .child(
                div().child("Wrap").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(
                            text()
                                .wrap(Wrap::Nowrap)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .wrap(Wrap::Wrap)
                                .child("The quick brown fox jumps over the lazy dog."),
                        ),
                ),
            )
            .child(
                div().child("Color").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(
                            text()
                                .color(AccentColor::Indigo)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .color(AccentColor::Cyan)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .color(AccentColor::Orange)
                                .child("The quick brown fox jumps over the lazy dog."),
                        )
                        .child(
                            text()
                                .color(AccentColor::Crimson)
                                .child("The quick brown fox jumps over the lazy dog."),
                        ),
                ),
            )
    }
}

fn main() {
    run_app(|_cx| Main {});
}
