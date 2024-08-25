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
use gpux_radix_themes::typography::{code, CodeVariant};
use gpux_radix_themes::typography::{Size, Weight};

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
                        .items_start()
                        .gap_2()
                        .child(code().size(Size::One).child("console.log()"))
                        .child(code().size(Size::Two).child("console.log()"))
                        .child(code().size(Size::Three).child("console.log()"))
                        .child(code().size(Size::Four).child("console.log()"))
                        .child(code().size(Size::Five).child("console.log()"))
                        .child(code().size(Size::Six).child("console.log()"))
                        .child(code().size(Size::Seven).child("console.log()"))
                        .child(code().size(Size::Eight).child("console.log()"))
                        .child(code().size(Size::Nine).child("console.log()")),
                ),
            )
            .child(
                div().child("Variant").child(
                    div()
                        .stack_v()
                        .items_start()
                        .gap_2()
                        .child(code().variant(CodeVariant::Solid).child("console.log()"))
                        .child(code().variant(CodeVariant::Soft).child("console.log()"))
                        .child(code().variant(CodeVariant::Outline).child("console.log()"))
                        .child(code().variant(CodeVariant::Ghost).child("console.log()")),
                ),
            )
            .child(
                div().child("Color").child(
                    div()
                        .stack_v()
                        .items_start()
                        .gap_2()
                        .child(code().color(AccentColor::Indigo).child("console.log()"))
                        .child(code().color(AccentColor::Cyan).child("console.log()"))
                        .child(code().color(AccentColor::Orange).child("console.log()"))
                        .child(code().color(AccentColor::Crimson).child("console.log()")),
                ),
            )
            .child(
                div().child("Weight").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(code().weight(Weight::Regular).child("console.log()"))
                        .child(code().weight(Weight::Bold).child("console.log()")),
                ),
            )
    }
}

fn main() {
    run_app(|_cx| Main {});
}
