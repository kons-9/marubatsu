pub mod ai;
pub mod human;
pub mod random;

use crate::environment::Board;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AgentType {
    MARU,
    BATSU,
}

pub enum Command {
    Next(usize, usize),
    Prev,
    Help,
    Exit,
}

impl AgentType {
    pub fn next(&self) -> Self {
        match self {
            Self::MARU => Self::BATSU,
            Self::BATSU => Self::MARU,
        }
    }
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MARU => write!(f, "○"),
            Self::BATSU => write!(f, "×"),
        }
    }
}

pub trait AgentTrait {
    fn agent_type(&self) -> AgentType;
    fn next(&self, board: &Board) -> Command;
}
