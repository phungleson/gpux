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
    div, px, rems, AnyElement, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use smallvec::{smallvec, SmallVec};

use gpux_css::box_shadow::box_shadow;
use gpux_css::stack_ext::StackExt;

use crate::theme::Theme;
use crate::typography::Size;

#[derive(IntoElement, Default)]
pub struct Kbd {
    children: SmallVec<[AnyElement; 2]>,
    size: Size,
}

impl Kbd {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ParentElement for Kbd {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Kbd {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .stack_h()
            .font_weight(theme.font_weight_regular)
            .whitespace_nowrap()
            .px(rems(0.5))
            .pb(rems(0.05))
            .rounded(rems(0.35))
            .text_color(theme.gray().solid.step_12())
            .bg(theme.gray().solid.step_1())
            .line_height(rems(1.7))
            .pt(px(4.))
            .shadow(smallvec![
                box_shadow(
                    0.,
                    rems(-0.05).to_pixels(cx.rem_size()),
                    0.,
                    0.,
                    theme.gray().transparent.step_2(),
                    true,
                ),
                box_shadow(
                    0.,
                    rems(0.05).to_pixels(cx.rem_size()),
                    0.,
                    0.,
                    theme.white.step_12(),
                    true,
                ),
                box_shadow(
                    0.,
                    rems(0.25).to_pixels(cx.rem_size()),
                    0.,
                    0.,
                    theme.gray().transparent.step_2(),
                    true,
                ),
                box_shadow(
                    0.,
                    rems(-0.05).to_pixels(cx.rem_size()),
                    0.,
                    0.,
                    theme.gray().transparent.step_6(),
                    true,
                ),
                box_shadow(
                    0.,
                    0.,
                    0.,
                    rems(0.05).to_pixels(cx.rem_size()),
                    theme.gray().transparent.step_5(),
                    false,
                ),
                box_shadow(
                    0.,
                    rems(0.08).to_pixels(cx.rem_size()),
                    rems(0.17).to_pixels(cx.rem_size()),
                    0.,
                    theme.gray().transparent.step_7(),
                    false,
                )
            ])
            .children(self.children)
    }
}

pub fn kbd() -> Kbd {
    Kbd::new()
}
