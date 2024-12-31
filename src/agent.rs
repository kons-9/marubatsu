#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Agent {
    MARU,
    BATSU,
}

impl Agent {
    pub fn next(&self) -> Self {
        match self {
            Self::MARU => Self::BATSU,
            Self::BATSU => Self::MARU,
        }
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MARU => write!(f, "○"),
            Self::BATSU => write!(f, "×"),
        }
    }
}
