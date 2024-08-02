use gpui::{
    div, px, BoxShadow, ElementId, IntoElement, ParentElement, RenderOnce, Styled, WindowContext,
};
use gpux_css::box_shadow::box_shadows_str;
use once_cell::sync::OnceCell;
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct NesJp {
    pub id: ElementId,
}

impl NesJp {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

static BOX_SHADOWS: OnceCell<SmallVec<[BoxShadow; 2]>> = OnceCell::new();

impl RenderOnce for NesJp {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let box_shadows = BOX_SHADOWS.get_or_init(|| {
            let color = "#871f37";
            box_shadows_str(
                format!(
                    r#"
                    28px 4px #333,
                    28px 8px #333,
                    32px 12px #333,
                    4px 16px {color},
                    8px 16px {color},
                    12px 16px {color},
                    16px 16px {color},
                    20px 16px {color},
                    24px 16px {color},
                    28px 16px {color},
                    32px 16px {color},
                    36px 16px {color},
                    40px 16px {color},
                    44px 16px {color},
                    48px 16px {color},
                    52px 16px {color},
                    56px 16px {color},
                    60px 16px {color},
                    4px 20px {color},
                    8px 20px #dfd3b9,
                    12px 20px #dfd3b9,
                    16px 20px #dfd3b9,
                    20px 20px #dfd3b9,
                    24px 20px #dfd3b9,
                    28px 20px {color},
                    32px 20px {color},
                    36px 20px {color},
                    40px 20px {color},
                    44px 20px {color},
                    48px 20px {color},
                    52px 20px {color},
                    56px 20px {color},
                    60px 20px {color},
                    4px 24px {color},
                    8px 24px #dfd3b9,
                    12px 24px #dfd3b9,
                    16px 24px #333,
                    20px 24px #dfd3b9,
                    24px 24px #dfd3b9,
                    28px 24px {color},
                    32px 24px {color},
                    36px 24px {color},
                    40px 24px {color},
                    44px 24px {color},
                    48px 24px {color},
                    52px 24px {color},
                    56px 24px {color},
                    60px 24px {color},
                    4px 28px {color},
                    8px 28px #dfd3b9,
                    12px 28px #333,
                    16px 28px #333,
                    20px 28px #333,
                    24px 28px #dfd3b9,
                    28px 28px #dfd3b9,
                    32px 28px #dfd3b9,
                    36px 28px #dfd3b9,
                    40px 28px #dfd3b9,
                    44px 28px #dfd3b9,
                    48px 28px #dfd3b9,
                    52px 28px #dfd3b9,
                    56px 28px #dfd3b9,
                    60px 28px {color},
                    4px 32px {color},
                    8px 32px #dfd3b9,
                    12px 32px #dfd3b9,
                    16px 32px #333,
                    20px 32px #dfd3b9,
                    24px 32px #dfd3b9,
                    28px 32px #333,
                    32px 32px {color},
                    36px 32px #333,
                    40px 32px #dfd3b9,
                    44px 32px #333,
                    48px 32px #dfd3b9,
                    52px 32px #333,
                    56px 32px #dfd3b9,
                    60px 32px {color},
                    4px 36px {color},
                    8px 36px #dfd3b9,
                    12px 36px #dfd3b9,
                    16px 36px #dfd3b9,
                    20px 36px #dfd3b9,
                    24px 36px #dfd3b9,
                    28px 36px #dfd3b9,
                    32px 36px #dfd3b9,
                    36px 36px #dfd3b9,
                    40px 36px #dfd3b9,
                    44px 36px #dfd3b9,
                    48px 36px #dfd3b9,
                    52px 36px #dfd3b9,
                    56px 36px #dfd3b9,
                    60px 36px {color},
                    4px 40px {color},
                    8px 40px {color},
                    12px 40px {color},
                    16px 40px {color},
                    20px 40px {color},
                    24px 40px {color},
                    28px 40px {color},
                    32px 40px {color},
                    36px 40px {color},
                    40px 40px {color},
                    44px 40px {color},
                    48px 40px {color},
                    52px 40px {color},
                    56px 40px {color},
                    60px 40px {color}
                    "#,
                ).as_str(),
            )
        });

        div().min_w(px(60.)).min_h(px(40.)).relative().child(
            div()
                .absolute()
                .top(px(-4.))
                .left(px(-4.))
                .size(px(4.))
                .shadow(box_shadows.clone()),
        )
    }
}
