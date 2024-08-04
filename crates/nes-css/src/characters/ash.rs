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
    div, px, BoxShadow, ElementId, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use gpux_css::box_shadow::box_shadows_str;
use once_cell::sync::OnceCell;
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct Ash {
    pub id: ElementId,
}

impl Ash {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

static BOX_SHADOWS: OnceCell<SmallVec<[BoxShadow; 2]>> = OnceCell::new();

impl RenderOnce for Ash {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let box_shadows = BOX_SHADOWS.get_or_init(|| {
            let color = "#181818";
            box_shadows_str(
                format!(
                    r#"
                    30px 6px {color},
                    36px 6px {color},
                    42px 6px {color},
                    48px 6px {color},
                    54px 6px {color},
                    60px 6px {color},
                    24px 12px {color},
                    30px 12px #ff614e,
                    36px 12px #ff614e,
                    42px 12px #ff614e,
                    48px 12px #f8f8ff,
                    54px 12px #f8f8ff,
                    60px 12px #007f7f,
                    66px 12px {color},
                    18px 18px {color},
                    24px 18px #ff614e,
                    30px 18px #ff614e,
                    36px 18px #ff614e,
                    42px 18px #ff614e,
                    48px 18px #f8f8ff,
                    54px 18px #f8f8ff,
                    60px 18px #f8f8ff,
                    66px 18px #f8f8ff,
                    72px 18px {color},
                    18px 24px {color},
                    24px 24px #ff614e,
                    30px 24px #ff614e,
                    36px 24px #ff614e,
                    42px 24px #ff614e,
                    48px 24px #f8f8ff,
                    54px 24px #f8f8ff,
                    60px 24px #007f7f,
                    66px 24px #007f7f,
                    72px 24px {color},
                    78px 24px {color},
                    12px 30px {color},
                    18px 30px {color},
                    24px 30px {color},
                    30px 30px #ff614e,
                    36px 30px #ff614e,
                    42px 30px #ff614e,
                    48px 30px #ff614e,
                    54px 30px #ff614e,
                    60px 30px #ff614e,
                    66px 30px #ff614e,
                    72px 30px #ff614e,
                    78px 30px #ff614e,
                    84px 30px {color},
                    12px 36px {color},
                    18px 36px {color},
                    24px 36px {color},
                    30px 36px {color},
                    36px 36px {color},
                    42px 36px {color},
                    48px 36px {color},
                    54px 36px #ff614e,
                    60px 36px #ff614e,
                    66px 36px #ff614e,
                    72px 36px {color},
                    78px 36px {color},
                    6px 42px {color},
                    12px 42px {color},
                    18px 42px {color},
                    24px 42px {color},
                    30px 42px {color},
                    36px 42px {color},
                    42px 42px {color},
                    48px 42px #ffe3c5,
                    54px 42px #ffe3c5,
                    60px 42px {color},
                    66px 42px #ffe3c5,
                    72px 42px {color},
                    12px 48px {color},
                    18px 48px {color},
                    24px 48px #ffe3c5,
                    30px 48px #ffe3c5,
                    36px 48px #ffe3c5,
                    42px 48px {color},
                    48px 48px #ffe3c5,
                    54px 48px #ffe3c5,
                    60px 48px {color},
                    66px 48px #ffe3c5,
                    72px 48px {color},
                    18px 54px {color},
                    24px 54px #ffe3c5,
                    30px 54px #ffe3c5,
                    36px 54px #ffe3c5,
                    42px 54px #ffe3c5,
                    48px 54px #ffe3c5,
                    54px 54px #ffe3c5,
                    60px 54px #ffe3c5,
                    66px 54px #ffe3c5,
                    72px 54px {color},
                    12px 60px {color},
                    18px 60px #4169e1,
                    24px 60px {color},
                    30px 60px {color},
                    36px 60px #ffe3c5,
                    42px 60px #ffe3c5,
                    48px 60px #ffe3c5,
                    54px 60px #ffe3c5,
                    60px 60px #ffe3c5,
                    66px 60px {color},
                    12px 66px {color},
                    18px 66px #4169e1,
                    24px 66px {color},
                    30px 66px {color},
                    36px 66px {color},
                    42px 66px {color},
                    48px 66px {color},
                    54px 66px {color},
                    60px 66px {color},
                    12px 72px {color},
                    18px 72px {color},
                    24px 72px #ffe3c5,
                    30px 72px #ffe3c5,
                    36px 72px {color},
                    42px 72px #4169e1,
                    48px 72px #4169e1,
                    54px 72px {color},
                    60px 72px {color},
                    66px 72px {color},
                    6px 78px {color},
                    12px 78px #007f7f,
                    18px 78px {color},
                    24px 78px #ffe3c5,
                    30px 78px #ffe3c5,
                    36px 78px {color},
                    42px 78px #4169e1,
                    48px 78px {color},
                    54px 78px {color},
                    60px 78px #007f7f,
                    66px 78px #007f7f,
                    72px 78px {color},
                    6px 84px {color},
                    12px 84px #007f7f,
                    18px 84px #007f7f,
                    24px 84px {color},
                    30px 84px {color},
                    36px 84px {color},
                    42px 84px {color},
                    48px 84px {color},
                    54px 84px #007f7f,
                    60px 84px #007f7f,
                    66px 84px {color},
                    12px 90px {color},
                    18px 90px {color},
                    54px 90px {color},
                    60px 90px {color}
                    "#,
                )
                .as_str(),
            )
        });

        div().min_h(px(90.)).min_w(px(96.)).relative().child(
            div()
                .absolute()
                .top(px(-6.))
                .left(px(-6.))
                .size(px(6.))
                .shadow(box_shadows.clone()),
        )
    }
}
