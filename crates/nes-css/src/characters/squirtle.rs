use gpui::{
    div, px, BoxShadow, ElementId, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use gpux_css::box_shadow::box_shadows_str;
use once_cell::sync::OnceCell;
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct Squirtle {
    pub id: ElementId,
}

impl Squirtle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

static BOX_SHADOWS: OnceCell<SmallVec<[BoxShadow; 2]>> = OnceCell::new();

impl RenderOnce for Squirtle {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let box_shadows = BOX_SHADOWS.get_or_init(|| {
            let color = "#9cf";
            box_shadows_str(
                format!(
                    r#"
                    24px 6px #000,
                    30px 6px #000,
                    36px 6px #000,
                    42px 6px #000,
                    102px 6px #000,
                    108px 6px #000,
                    114px 6px #000,
                    18px 12px #000,
                    24px 12px {color},
                    30px 12px {color},
                    36px 12px {color},
                    42px 12px {color},
                    48px 12px #000,
                    54px 12px #000,
                    96px 12px #000,
                    102px 12px {color},
                    108px 12px {color},
                    114px 12px {color},
                    120px 12px #000,
                    12px 18px #000,
                    18px 18px {color},
                    24px 18px {color},
                    30px 18px {color},
                    36px 18px {color},
                    42px 18px {color},
                    48px 18px {color},
                    54px 18px {color},
                    60px 18px #000,
                    66px 18px #000,
                    90px 18px #000,
                    96px 18px {color},
                    102px 18px {color},
                    108px 18px {color},
                    114px 18px {color},
                    120px 18px {color},
                    126px 18px #000,
                    12px 24px #000,
                    18px 24px {color},
                    24px 24px {color},
                    30px 24px {color},
                    36px 24px {color},
                    42px 24px {color},
                    48px 24px {color},
                    54px 24px {color},
                    60px 24px #000,
                    66px 24px #f89934,
                    72px 24px #000,
                    78px 24px #000,
                    90px 24px #000,
                    96px 24px {color},
                    102px 24px {color},
                    108px 24px {color},
                    114px 24px #000,
                    120px 24px {color},
                    126px 24px #000,
                    6px 30px #000,
                    12px 30px #cb6633,
                    18px 30px {color},
                    24px 30px {color},
                    30px 30px {color},
                    36px 30px {color},
                    42px 30px {color},
                    48px 30px {color},
                    54px 30px {color},
                    60px 30px {color},
                    66px 30px #f89934,
                    72px 30px #f89934,
                    78px 30px #f89934,
                    84px 30px #000,
                    90px 30px {color},
                    96px 30px {color},
                    102px 30px {color},
                    108px 30px #000,
                    114px 30px {color},
                    120px 30px {color},
                    126px 30px #000,
                    6px 36px #000,
                    12px 36px {color},
                    18px 36px {color},
                    24px 36px {color},
                    30px 36px {color},
                    36px 36px #fff,
                    42px 36px #000,
                    48px 36px {color},
                    54px 36px {color},
                    60px 36px {color},
                    66px 36px #fff,
                    72px 36px #f89934,
                    78px 36px #f89934,
                    84px 36px #f89934,
                    90px 36px #000,
                    96px 36px {color},
                    102px 36px {color},
                    108px 36px #000,
                    114px 36px {color},
                    120px 36px #000,
                    6px 42px #000,
                    12px 42px {color},
                    18px 42px {color},
                    24px 42px {color},
                    30px 42px {color},
                    36px 42px #000,
                    42px 42px #cb6633,
                    48px 42px {color},
                    54px 42px {color},
                    60px 42px {color},
                    66px 42px #fff,
                    72px 42px #f89934,
                    78px 42px #f89934,
                    84px 42px #f89934,
                    90px 42px #000,
                    96px 42px {color},
                    102px 42px #000,
                    108px 42px #000,
                    114px 42px #000,
                    12px 48px #000,
                    18px 48px {color},
                    24px 48px {color},
                    30px 48px {color},
                    36px 48px #000,
                    42px 48px #cb6633,
                    48px 48px {color},
                    54px 48px {color},
                    60px 48px {color},
                    66px 48px #000,
                    72px 48px #fff,
                    78px 48px #f89934,
                    84px 48px #f89934,
                    90px 48px #f89934,
                    96px 48px #000,
                    102px 48px #000,
                    18px 54px #000,
                    24px 54px #000,
                    30px 54px {color},
                    36px 54px {color},
                    42px 54px {color},
                    48px 54px {color},
                    54px 54px #000,
                    60px 54px #000,
                    66px 54px {color},
                    72px 54px {color},
                    78px 54px #fff,
                    84px 54px #f89934,
                    90px 54px #f89934,
                    96px 54px #000,
                    18px 60px #000,
                    24px 60px {color},
                    30px 60px #000,
                    36px 60px #000,
                    42px 60px #000,
                    48px 60px #000,
                    54px 60px {color},
                    60px 60px {color},
                    66px 60px {color},
                    72px 60px {color},
                    78px 60px #fff,
                    84px 60px #f89934,
                    90px 60px #f89934,
                    96px 60px #000,
                    24px 66px #000,
                    30px 66px #000,
                    36px 66px #ff3,
                    42px 66px #ff3,
                    48px 66px #000,
                    54px 66px {color},
                    60px 66px {color},
                    66px 66px {color},
                    72px 66px #000,
                    78px 66px #fff,
                    84px 66px #f89934,
                    90px 66px #f89934,
                    96px 66px #000,
                    36px 72px #000,
                    42px 72px #ff3,
                    48px 72px #ff3,
                    54px 72px #000,
                    60px 72px #000,
                    66px 72px #000,
                    72px 72px #000,
                    78px 72px #fff,
                    84px 72px #f89934,
                    90px 72px #f89934,
                    96px 72px #000,
                    30px 78px #000,
                    36px 78px {color},
                    42px 78px #000,
                    48px 78px #ff3,
                    54px 78px #ff3,
                    60px 78px #ff3,
                    66px 78px #ff3,
                    72px 78px #ff3,
                    78px 78px #000,
                    84px 78px #fff,
                    90px 78px #000,
                    36px 84px #000,
                    42px 84px #000,
                    48px 84px #000,
                    54px 84px #000,
                    60px 84px #ff3,
                    66px 84px #ff3,
                    72px 84px {color},
                    78px 84px #000,
                    84px 84px #fff,
                    90px 84px #000,
                    54px 90px #000,
                    60px 90px #000,
                    66px 90px #000,
                    72px 90px {color},
                    78px 90px #000,
                    84px 90px #000,
                    60px 96px #000,
                    66px 96px {color},
                    72px 96px {color},
                    78px 96px {color},
                    84px 96px #000,
                    66px 102px #000,
                    72px 102px #000,
                    78px 102px #000
                    "#,
                )
                    .as_str(),
            )
        });

        div().min_w(px(126.)).min_h(px(102.)).relative().child(
            div()
                .absolute()
                .top(px(-6.))
                .left(px(-6.))
                .size(px(6.))
                .shadow(box_shadows.clone()),
        )
    }
}