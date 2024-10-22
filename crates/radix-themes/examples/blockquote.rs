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

use crate::app::run_app;
use gpux_css::color::white;
use gpux_css::stack_ext::StackExt;
use gpux_radix_themes::theme::AccentColor;
use gpux_radix_themes::typography::blockquote;
use gpux_radix_themes::typography::Wrap;
use gpux_radix_themes::typography::{Size, Weight};

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
                            blockquote()
                                .size(Size::One)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Two)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Three)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Four)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Five)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Six)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Seven)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Eight)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .size(Size::Nine)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        ),
                ),
            )
            .child(
                div().child("Weight").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(
                            blockquote()
                                .weight(Weight::Regular)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .weight(Weight::Medium)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .weight(Weight::Bold)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        ),
                ),
            )
            .child(
                div().child("Wrap").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(
                            blockquote()
                                .wrap(Wrap::Nowrap)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .wrap(Wrap::Wrap)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        ),
                ),
            )
            .child(
                div().child("Color").child(
                    div()
                        .stack_v()
                        .gap_2()
                        .child(
                            blockquote()
                                .color(AccentColor::Indigo)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .color(AccentColor::Cyan)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .color(AccentColor::Orange)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        )
                        .child(
                            blockquote()
                                .color(AccentColor::Crimson)
                                .child("Perfect typography is certainly the most elusive of all arts. Sculpture in stone alone comes near it in obstinacy."),
                        ),
                ),
            )
    }
}

fn main() {
    run_app(|_cx| Main {});
}
