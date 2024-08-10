use gpui::Hsla;

pub trait HslaExt {
    fn brightness(&self, amount: f32) -> Hsla;

    fn saturate(&self, amount: f32) -> Hsla;
}

impl HslaExt for Hsla {
    fn brightness(&self, amount: f32) -> Hsla {
        Hsla {
            h: self.h,
            s: self.s,
            l: self.l * amount,
            a: self.a,
        }
    }

    fn saturate(&self, amount: f32) -> Hsla {
        Hsla {
            h: self.h,
            s: self.s * amount,
            l: self.l,
            a: self.a,
        }
    }
}
