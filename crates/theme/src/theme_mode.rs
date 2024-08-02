#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}
