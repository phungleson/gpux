use std::fmt::Display;

/// Represents the selection status of an element.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Selection {
    /// The element is not selected.
    #[default]
    Unselected,
    /// The selection state of the element is indeterminate.
    Indeterminate,
    /// The element is selected.
    Selected,
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unselected => write!(f, "Unselected"),
            Self::Indeterminate => write!(f, "Indeterminate"),
            Self::Selected => write!(f, "Selected"),
        }
    }
}

impl Selection {
    /// Returns the inverse of the current selection status.
    ///
    /// Indeterminate states become selected if inverted.
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }
}
