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
