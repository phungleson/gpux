use gpui::Hsla;
use helpers::css::{hsl, hsla};

/// A gorgeous, accessible color system for user interfaces.
/// There are 12 steps in each scale.
/// https://www.radix-ui.com/colors
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    pub step_1: Hsla,
    pub step_2: Hsla,
    pub step_3: Hsla,
    pub step_4: Hsla,
    pub step_5: Hsla,
    pub step_6: Hsla,
    pub step_7: Hsla,
    pub step_8: Hsla,
    pub step_9: Hsla,
    pub step_10: Hsla,
    pub step_11: Hsla,
    pub step_12: Hsla,
}

impl Colors {
    pub fn white() -> Hsla {
        hsl(0., 0., 100.)
    }

    pub fn indigo() -> Colors {
        Colors {
            step_1: hsl(240., 33., 99.),
            step_2: hsl(225., 100., 98.),
            step_3: hsl(222., 89., 96.),
            step_4: hsl(224., 100., 94.),
            step_5: hsl(224., 100., 91.),
            step_6: hsl(225., 100., 88.),
            step_7: hsl(226., 87., 82.),
            step_8: hsl(226., 75., 75.),
            step_9: hsl(226., 70., 55.),
            step_10: hsl(226., 65., 52.),
            step_11: hsl(226., 56., 50.),
            step_12: hsl(226., 50., 24.),
        }
    }

    pub fn indigo_alpha() -> Colors {
        Colors {
            step_1: hsla(240., 100., 25., 0.01),
            step_2: hsla(225., 100., 50., 0.03),
            step_3: hsla(222., 100., 47., 0.07),
            step_4: hsla(224., 100., 50., 0.12),
            step_5: hsla(224., 100., 50., 0.18),
            step_6: hsla(225., 100., 50., 0.24),
            step_7: hsla(226., 100., 46., 0.33),
            step_8: hsla(226., 100., 43., 0.45),
            step_9: hsla(226., 100., 41., 0.76),
            step_10: hsla(226., 100., 39., 0.80),
            step_11: hsla(226., 100., 36., 0.77),
            step_12: hsla(226., 100., 14., 0.88),
        }
    }

    pub fn gray() -> Colors {
        Colors {
            step_1: hsl(0., 0., 7.),
            step_2: hsl(0., 0., 10.),
            step_3: hsl(0., 0., 13.),
            step_4: hsl(0., 0., 16.),
            step_5: hsl(0., 0., 19.),
            step_6: hsl(0., 0., 23.),
            step_7: hsl(0., 0., 28.),
            step_8: hsl(0., 0., 38.),
            step_9: hsl(0., 0., 43.),
            step_10: hsl(0., 0., 48.),
            step_11: hsl(0., 0., 71.),
            step_12: hsl(0., 0., 93.),
        }
    }

    pub fn gray_alpha() -> Colors {
        Colors {
            step_1: hsla(0., 0., 0., 0.00),
            step_2: hsla(0., 0., 100., 0.04),
            step_3: hsla(0., 0., 100., 0.07),
            step_4: hsla(0., 0., 100., 0.11),
            step_5: hsla(0., 0., 100., 0.13),
            step_6: hsla(0., 0., 100., 0.17),
            step_7: hsla(0., 0., 100., 0.23),
            step_8: hsla(0., 0., 100., 0.33),
            step_9: hsla(0., 0., 100., 0.39),
            step_10: hsla(0., 0., 100., 0.45),
            step_11: hsla(0., 0., 100., 0.69),
            step_12: hsla(0., 0., 100., 0.93),
        }
    }

    pub fn slate() -> Colors {
        Colors {
            step_1: hsl(240., 20., 99.),
            step_2: hsl(240., 20., 98.),
            step_3: hsl(240., 11., 95.),
            step_4: hsl(240., 10., 92.),
            step_5: hsl(230., 11., 89.),
            step_6: hsl(240., 10., 86.),
            step_7: hsl(233., 10., 82.),
            step_8: hsl(231., 10., 75.),
            step_9: hsl(231., 6., 57.),
            step_10: hsl(226., 5., 53.),
            step_11: hsl(220., 6., 40.),
            step_12: hsl(210., 13., 13.),
        }
    }

    pub fn slate_alpha() -> Colors {
        Colors {
            step_1: hsla(240., 100., 17., 0.01),
            step_2: hsla(240., 100., 17., 0.02),
            step_3: hsla(240., 100., 10., 0.06),
            step_4: hsla(240., 100., 9., 0.09),
            step_5: hsla(229., 100., 10., 0.12),
            step_6: hsla(240., 100., 9., 0.15),
            step_7: hsla(232., 100., 9., 0.20),
            step_8: hsla(230., 100., 9., 0.27),
            step_9: hsla(230., 100., 6., 0.45),
            step_10: hsla(224., 100., 5., 0.50),
            step_11: hsla(219., 100., 4., 0.62),
            step_12: hsla(207., 100., 2., 0.89),
        }
    }
}
