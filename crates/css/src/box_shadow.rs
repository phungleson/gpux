use gpui::{point, BoxShadow, Hsla, Pixels};

/// Make a BoxShadow like CSS
///
/// e.g:
///
/// If CSS is `box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.1);`
///
/// Then the equivalent in Rust is `box_shadow(0., 0., 10., 0., hsla(0., 0., 0., 0.1))`
pub fn box_shadow(
    x: impl Into<Pixels>,
    y: impl Into<Pixels>,
    blur_radius: impl Into<Pixels>,
    spread_radius: impl Into<Pixels>,
    color: impl Into<Hsla>,
) -> BoxShadow {
    BoxShadow {
        offset: point(x.into(), y.into()),
        blur_radius: blur_radius.into(),
        spread_radius: spread_radius.into(),
        color: color.into(),
    }
}
