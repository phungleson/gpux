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
pub struct Pokeball {
    pub id: ElementId,
}

impl Pokeball {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

static BOX_SHADOWS: OnceCell<SmallVec<[BoxShadow; 2]>> = OnceCell::new();

impl RenderOnce for Pokeball {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let box_shadows = BOX_SHADOWS.get_or_init(|| {
            let color = "#060606";
            box_shadows_str(
                format!(
                    r#"
                    36px 6px {color},
                    42px 6px {color},
                    48px 6px {color},
                    54px 6px {color},
                    24px 12px {color},
                    30px 12px {color},
                    36px 12px #ff001d,
                    42px 12px #ff001d,
                    48px 12px #ff001d,
                    54px 12px #ff001d,
                    60px 12px {color},
                    66px 12px {color},
                    18px 18px {color},
                    24px 18px #fff,
                    30px 18px #fff,
                    36px 18px #ff001d,
                    42px 18px #ff001d,
                    48px 18px #ff001d,
                    54px 18px #ff001d,
                    60px 18px #ff001d,
                    66px 18px #ff001d,
                    72px 18px {color},
                    12px 24px {color},
                    18px 24px #fff,
                    24px 24px #fff,
                    30px 24px #ff001d,
                    36px 24px #ff001d,
                    42px 24px #ff001d,
                    48px 24px #ff001d,
                    54px 24px #ff001d,
                    60px 24px #ff001d,
                    66px 24px #ff001d,
                    72px 24px {color},
                    78px 24px {color},
                    12px 30px {color},
                    18px 30px #fff,
                    24px 30px #ff001d,
                    30px 30px #ff001d,
                    36px 30px #ff001d,
                    42px 30px #ff001d,
                    48px 30px #ff001d,
                    54px 30px #ff001d,
                    60px 30px #ff001d,
                    66px 30px #ff001d,
                    72px 30px #ff001d,
                    78px 30px {color},
                    6px 36px {color},
                    12px 36px #fff,
                    18px 36px #ff001d,
                    24px 36px #ff001d,
                    30px 36px #ff001d,
                    36px 36px #ff001d,
                    42px 36px #ff001d,
                    48px 36px #ff001d,
                    54px 36px #ff001d,
                    60px 36px #ff001d,
                    66px 36px #ff001d,
                    72px 36px #ff001d,
                    78px 36px #ff001d,
                    84px 36px {color},
                    6px 42px {color},
                    12px 42px {color},
                    18px 42px {color},
                    24px 42px #ff001d,
                    30px 42px #ff001d,
                    36px 42px #ff001d,
                    42px 42px #ff001d,
                    48px 42px {color},
                    54px 42px {color},
                    60px 42px {color},
                    66px 42px #ff001d,
                    72px 42px #ff001d,
                    78px 42px #ff001d,
                    84px 42px {color},
                    6px 48px {color},
                    12px 48px {color},
                    18px 48px {color},
                    24px 48px {color},
                    30px 48px #ff001d,
                    36px 48px #ff001d,
                    42px 48px {color},
                    48px 48px #fff,
                    54px 48px #fff,
                    60px 48px #fff,
                    66px 48px {color},
                    72px 48px #ff001d,
                    78px 48px #ff001d,
                    84px 48px {color},
                    6px 54px {color},
                    12px 54px #9fa1a1,
                    18px 54px #fff,
                    24px 54px {color},
                    30px 54px {color},
                    36px 54px {color},
                    42px 54px {color},
                    48px 54px #fff,
                    54px 54px #fff,
                    60px 54px #fff,
                    66px 54px {color},
                    72px 54px {color},
                    78px 54px {color},
                    84px 54px {color},
                    12px 60px {color},
                    18px 60px #fff,
                    24px 60px #fff,
                    30px 60px #fff,
                    36px 60px {color},
                    42px 60px {color},
                    48px 60px #fff,
                    54px 60px #fff,
                    60px 60px #fff,
                    66px 60px {color},
                    72px 60px #fff,
                    78px 60px {color},
                    12px 66px {color},
                    18px 66px #fff,
                    24px 66px #fff,
                    30px 66px #fff,
                    36px 66px #fff,
                    42px 66px #fff,
                    48px 66px {color},
                    54px 66px {color},
                    60px 66px {color},
                    66px 66px #fff,
                    72px 66px #fff,
                    78px 66px {color},
                    18px 72px {color},
                    24px 72px #9fa1a1,
                    30px 72px #9fa1a1,
                    36px 72px #fff,
                    42px 72px #fff,
                    48px 72px #fff,
                    54px 72px #fff,
                    60px 72px #fff,
                    66px 72px #fff,
                    72px 72px {color},
                    24px 78px {color},
                    30px 78px {color},
                    36px 78px #9fa1a1,
                    42px 78px #9fa1a1,
                    48px 78px #9fa1a1,
                    54px 78px #9fa1a1,
                    60px 78px {color},
                    66px 78px {color},
                    36px 84px {color},
                    42px 84px {color},
                    48px 84px {color},
                    54px 84px {color}
                    "#,
                )
                .as_str(),
            )
        });

        div().min_h(px(84.)).min_w(px(84.)).relative().child(
            div()
                .absolute()
                .top(px(-6.))
                .left(px(-6.))
                .size(px(6.))
                .shadow(box_shadows.clone()),
        )
    }
}
