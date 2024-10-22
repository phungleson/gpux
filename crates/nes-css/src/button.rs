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
    div, px, rgb, ElementId, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use smallvec::smallvec;

use gpux_css::box_shadow::box_shadow;

#[derive(IntoElement)]
pub struct Button {
    pub id: ElementId,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().relative().child("Normal").child(
            div()
                .absolute()
                .top(px(-4.))
                .left(px(-4.))
                .right(px(-4.))
                .bottom(px(-4.))
                .shadow(smallvec![box_shadow(-4., -4., 0., 0., rgb(0xadafbc), true)]),
        )
    }
}
