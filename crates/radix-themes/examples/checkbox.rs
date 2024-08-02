use gpui::{
    App, Bounds, div, IntoElement, ParentElement, px, Render, size, Styled, ViewContext,
    VisualContext, WindowBounds, WindowOptions,
};

use gpux_css::color::white;
use gpux_interactivity::{disableable::Disableable, selection::Selection};
use gpux_radix_themes::{assets::Assets, checkbox::Checkbox, theme::Theme};
use gpux_radix_themes::checkbox::{CheckboxSize, CheckboxVariant};
use gpux_radix_themes::theme::{AccentColor, GrayColor};
use gpux_theme::theme_mode::ThemeMode;

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
            .child(div()
                .child("Variant")
                .child(div().flex_col()
                    .child(Checkbox::new("soft-selected").checked(Selection::Selected).variant(CheckboxVariant::Soft).label("Soft"))
                    .child(Checkbox::new("soft-not-selected").checked(Selection::Unselected).variant(CheckboxVariant::Soft))
                    .child(Checkbox::new("classic-selected").checked(Selection::Selected).variant(CheckboxVariant::Classic).label("Classic"))
                    .child(Checkbox::new("classic-not-selected").checked(Selection::Unselected).variant(CheckboxVariant::Classic))
                    .child(Checkbox::new("surface-selected").checked(Selection::Selected).variant(CheckboxVariant::Surface).label("Surface"))
                    .child(Checkbox::new("surface-not-selected").checked(Selection::Unselected).variant(CheckboxVariant::Surface)))
            )
            .child(div()
                .child("Color")
                .child(div().flex_col()
                    .child(Checkbox::new("indigo").checked(Selection::Selected).color(AccentColor::Indigo).label("Indigo").checked(self.checked))
                    .child(Checkbox::new("cyan").checked(Selection::Selected).color(AccentColor::Cyan).label("Cyan"))
                    .child(Checkbox::new("orange").checked(Selection::Selected).color(AccentColor::Orange).label("Orange"))
                    .child(Checkbox::new("crimson").checked(Selection::Selected).color(AccentColor::Crimson).label("Crimson")))
            )
            .child(div().child("Disabled")
                .child(div().flex_col()
                    .child(Checkbox::new("selected")
                        .label("Selected")
                        .checked(self.checked)
                        .on_click(cx.listener(|view, _, _| {
                            view.checked = view.checked.inverse();
                        }))
                    )
                    .child(Checkbox::new("not-selected").label("Not selected"))
                    .child(Checkbox::new("selected-disabled").checked(Selection::Selected).disabled(true).label("Selected"))
                    .child(Checkbox::new("not-selected-disabled").disabled(true).label("Not selected")))
            )
            .child(div()
                .child("Size")
                .child(div().flex_col()
                    .child(Checkbox::new("one")
                        .checked(Selection::Selected).size(CheckboxSize::One).label("One")
                    )
                    .child(Checkbox::new("two").checked(Selection::Selected).label("Two"))
                    .child(Checkbox::new("three")
                        .checked(Selection::Selected).size(CheckboxSize::Three).label("Three")
                    )
                )
            )
    }
}
fn main() {
    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        cx.set_global(Theme::new(ThemeMode::Light, AccentColor::Indigo, GrayColor::Slate));
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Main {
                    checked: Selection::Selected,
                })
            },
        ).unwrap();
    });
}
