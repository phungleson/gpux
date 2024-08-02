/// Represents the different modes that a theme can have.
#[derive(Debug, PartialEq, PartialOrd, Eq, Default, Copy, Clone)]
pub enum ThemeMode {
    /// Light theme mode.
    #[default]
    Light,
    /// Dark theme mode.
    Dark,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}
