#[derive(Debug, PartialEq, PartialOrd, Eq, Default, Copy, Clone)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}
