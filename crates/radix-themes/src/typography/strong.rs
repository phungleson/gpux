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

use gpui::{div, AnyElement, IntoElement, ParentElement, RenderOnce, Styled, WindowContext};
use smallvec::SmallVec;

use crate::theme::Theme;

#[derive(IntoElement, Default)]
pub struct Strong {
    children: SmallVec<[AnyElement; 2]>,
}

impl Strong {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ParentElement for Strong {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Strong {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .font_weight(theme.font_weight_bold)
            .children(self.children)
    }
}

pub fn strong() -> Strong {
    Strong::new()
}
