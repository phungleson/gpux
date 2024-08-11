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
use gpux_radix_themes::components::{dropdown_menu, DropdownMenuSize, DropdownMenuVariant};

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
                        .stack_h()
                        .gap_2()
                        .child(
                            dropdown_menu("one")
                                .variant(DropdownMenuVariant::Soft)
                                .size(DropdownMenuSize::One)
                                .label("Options"),
                        )
                        .child(
                            dropdown_menu("two")
                                .size(DropdownMenuSize::Two)
                                .label("Options"),
                        ),
                ),
            )
    }
}

fn main() {
    run_app(|_cx| Main {});
}
