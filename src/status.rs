use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Status {
    Incomplete,
    Complete,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incomplete => f.write_str("•"),
            Self::Complete => f.write_str("–"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_status_is_displayed_as_bullet() {
        assert_eq!(format!("{}", Status::Incomplete), "•");
    }

    #[test]
    fn complete_status_is_displayed_as_en_dash() {
        assert_eq!(format!("{}", Status::Complete), "–");
    }
}
