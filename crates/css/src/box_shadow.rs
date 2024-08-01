use gpui::{point, px, rgb, BoxShadow, Hsla, Pixels};
use regex::Regex;
use smallvec::SmallVec;

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
        inset: false,
    }
}

pub fn box_shadow_xy_color(
    x: impl Into<Pixels>,
    y: impl Into<Pixels>,
    color: impl Into<Hsla>,
) -> BoxShadow {
    BoxShadow {
        offset: point(x.into(), y.into()),
        blur_radius: px(0.),
        spread_radius: px(0.),
        color: color.into(),
        inset: false,
    }
}

pub fn box_shadow_xy_color_inset(
    x: impl Into<Pixels>,
    y: impl Into<Pixels>,
    color: impl Into<Hsla>,
    inset: bool,
) -> BoxShadow {
    BoxShadow {
        offset: point(x.into(), y.into()),
        blur_radius: px(0.),
        spread_radius: px(0.),
        color: color.into(),
        inset,
    }
}

// Parses 0px 0px #000000 into BoxShadow
pub fn box_shadow_str(str: &'_ str) -> Option<BoxShadow> {
    let regex = Regex::new(r"(\d+)px (\d+)px (#([0-9a-f]{3}){1,2})")
        .expect("box shadow str regex is invalid");

    regex.captures(str).map(|capture| {
        let (_, [x_str, y_str, color_str, _]) = capture.extract();
        let x = x_str.parse::<f32>().unwrap_or(0.);
        let y = y_str.parse::<f32>().unwrap_or(0.);
        dbg!(&color_str);
        let color = color_str.try_into().unwrap_or(rgb(0x000000));

        BoxShadow {
            offset: point(px(x), px(y)),
            blur_radius: px(0.),
            spread_radius: px(0.),
            color: color.into(),
            inset: false,
        }
    })
}

// Parses "0px 0px #000000, 0px 0px #000000,..." into SmallVec<BoxShadow>
pub fn box_shadows_str(str: &'_ str) -> SmallVec<[BoxShadow; 2]> {
    str.split(",")
        .flat_map(|str| box_shadow_str(str.trim()))
        .collect()
}
