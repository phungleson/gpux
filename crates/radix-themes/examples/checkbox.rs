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
use gpux_interactivity::{disableable::Disableable, selection::Selection};
use gpux_radix_themes::components::Checkbox;
use gpux_radix_themes::components::{CheckboxSize, CheckboxVariant};
use gpux_radix_themes::theme::AccentColor;

use crate::app::run_app;

mod app;

struct Main {
    checked: Selection,
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .p_4()
            .justify_center()
            .bg(white())
            .size_full()
            .child(
                div().child("Variant").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(
                            Checkbox::new("selected-soft")
                                .checked(Selection::Selected)
                                .variant(CheckboxVariant::Soft),
                        )
                        .child(
                            Checkbox::new("not-selected-soft")
                                .checked(Selection::Unselected)
                                .variant(CheckboxVariant::Soft),
                        )
                        .child(
                            Checkbox::new("selected-classic")
                                .checked(Selection::Selected)
                                .variant(CheckboxVariant::Classic),
                        )
                        .child(
                            Checkbox::new("not-selected-classic")
                                .checked(Selection::Unselected)
                                .variant(CheckboxVariant::Classic),
                        )
                        .child(
                            Checkbox::new("selected-surface")
                                .checked(Selection::Selected)
                                .variant(CheckboxVariant::Surface),
                        )
                        .child(
                            Checkbox::new("not-selected-surface")
                                .checked(Selection::Unselected)
                                .variant(CheckboxVariant::Surface),
                        ),
                ),
            )
            .child(
                div().child("Color").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(
                            Checkbox::new("indigo")
                                .checked(Selection::Selected)
                                .color(AccentColor::Indigo),
                        )
                        .child(
                            Checkbox::new("cyan")
                                .checked(Selection::Selected)
                                .color(AccentColor::Cyan),
                        )
                        .child(
                            Checkbox::new("orange")
                                .checked(Selection::Selected)
                                .color(AccentColor::Orange),
                        )
                        .child(
                            Checkbox::new("crimson")
                                .checked(Selection::Selected)
                                .color(AccentColor::Crimson),
                        ),
                ),
            )
            .child(
                div().child("High contrast").child(
                    div()
                        .stack_h()
                        .gap_2()
                        .child(
                            Checkbox::new("high-contrast-indigo")
                                .high_contrast(true)
                                .checked(Selection::Selected)
                                .color(AccentColor::Indigo),
                        )
                        .child(
                            Checkbox::new("high-contrast-cyan")
                                .high_contrast(true)
                                .checked(Selection::Selected)
                                .color(AccentColor::Cyan),
                        )
                        .child(
                            Checkbox::new("high-contrast-orange")
                                .high_contrast(true)
                                .checked(Selection::Selected)
                                .color(AccentColor::Orange),
                        )
                        .child(
                            Checkbox::new("high-contrast-crimson")
                                .high_contrast(true)
                                .checked(Selection::Selected)
                                .color(AccentColor::Crimson),
                        ),
                ),
            )
            .child(
                div().child("Disabled").child(
                    div()
                        .flex_col()
                        .child(
                            Checkbox::new("selected")
                                .label("Selected")
                                .checked(self.checked)
                                .on_click(cx.listener(|view, _, _| {
                                    view.checked = view.checked.inverse();
                                })),
                        )
                        .child(Checkbox::new("not-selected").label("Not selected"))
                        .child(
                            Checkbox::new("selected-disabled")
                                .checked(Selection::Selected)
                                .disabled(true)
                                .label("Selected"),
                        )
                        .child(
                            Checkbox::new("not-selected-disabled")
                                .disabled(true)
                                .label("Not selected"),
                        ),
                ),
            )
            .child(
                div().child("Size").child(
                    div()
                        .flex_col()
                        .child(
                            Checkbox::new("one")
                                .checked(Selection::Selected)
                                .size(CheckboxSize::One)
                                .label("One"),
                        )
                        .child(
                            Checkbox::new("two")
                                .checked(Selection::Selected)
                                .label("Two"),
                        )
                        .child(
                            Checkbox::new("three")
                                .checked(Selection::Selected)
                                .size(CheckboxSize::Three)
                                .label("Three"),
                        ),
                ),
            )
    }
}
fn main() {
    run_app(|_cx| Main {
        checked: Selection::Selected,
    });
}
