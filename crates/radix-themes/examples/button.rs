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

use gpui::{div, svg, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};

use gpux_css::color::white;
use gpux_css::stack_ext::StackExt;
use gpux_radix_themes::components::CheckboxIcon;
use gpux_radix_themes::components::{button, ButtonRadius, ButtonSize, ButtonVariant};
use gpux_radix_themes::theme::AccentColor;

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
                        .child(button("one").size(ButtonSize::One).label("Edit profile"))
                        .child(button("two").size(ButtonSize::Two).label("Edit profile"))
                        .child(
                            button("three")
                                .size(ButtonSize::Three)
                                .label("Edit profile"),
                        )
                        .child(button("four").size(ButtonSize::Four).label("Edit profile")),
                ),
            )
            .child(
                div().child("Variant").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(
                            button("classic")
                                .variant(ButtonVariant::Classic)
                                .label("Edit profile"),
                        )
                        .child(
                            button("solid")
                                .variant(ButtonVariant::Solid)
                                .label("Edit profile"),
                        )
                        .child(
                            button("soft")
                                .variant(ButtonVariant::Soft)
                                .label("Edit profile"),
                        )
                        .child(
                            button("surface")
                                .variant(ButtonVariant::Surface)
                                .label("Edit profile"),
                        )
                        .child(
                            button("outline")
                                .variant(ButtonVariant::Outline)
                                .label("Edit profile"),
                        ),
                ),
            )
            .child(
                div().child("Color").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(
                            button("indigo")
                                .color(AccentColor::Indigo)
                                .variant(ButtonVariant::Soft)
                                .label("Edit profile"),
                        )
                        .child(
                            button("cyan")
                                .color(AccentColor::Cyan)
                                .variant(ButtonVariant::Soft)
                                .label("Edit profile"),
                        )
                        .child(
                            button("orange")
                                .color(AccentColor::Orange)
                                .variant(ButtonVariant::Soft)
                                .label("Edit profile"),
                        )
                        .child(
                            button("crimson")
                                .color(AccentColor::Crimson)
                                .variant(ButtonVariant::Soft)
                                .label("Edit profile"),
                        ),
                ),
            )
            .child(
                div().child("Radius").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(
                            button("none")
                                .variant(ButtonVariant::Soft)
                                .radius(ButtonRadius::None)
                                .label("Edit profile"),
                        )
                        .child(
                            button("small")
                                .variant(ButtonVariant::Soft)
                                .radius(ButtonRadius::Small)
                                .label("Edit profile"),
                        )
                        .child(
                            button("medium")
                                .variant(ButtonVariant::Soft)
                                .radius(ButtonRadius::Medium)
                                .label("Edit profile"),
                        )
                        .child(
                            button("large")
                                .variant(ButtonVariant::Soft)
                                .radius(ButtonRadius::Large)
                                .label("Edit profile"),
                        )
                        .child(
                            button("full")
                                .variant(ButtonVariant::Soft)
                                .radius(ButtonRadius::Full)
                                .label("Edit profile"),
                        ),
                ),
            )
            .child(
                div().child("With icons").child(
                    div().stack_h().gap_2().child(
                        button("soft")
                            .variant(ButtonVariant::Soft)
                            .icon(svg().path(CheckboxIcon::Check.path()))
                            .label("Edit profile"),
                    ),
                ),
            )
            .child(
                div().child("Loading").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(button("loading").variant(ButtonVariant::Soft).loading(true)),
                ),
            )
            .child(
                div().child("Disabled").child(
                    div().stack_h().gap_2().child(
                        button("soft")
                            .variant(ButtonVariant::Soft)
                            .disabled(true)
                            .icon(svg().path(CheckboxIcon::Check.path()))
                            .label("Bookmark"),
                    ),
                ),
            )
    }
}

fn main() {
    run_app(|_cx| Main {});
}
