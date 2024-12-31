use crate::environment::Board;
use crate::environment::CellState;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AgentType {
    MARU,
    BATSU,
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
    fn next(&self, board: &Board) -> (usize, usize);
}

pub struct HumanAgent {
    pub agent_type: AgentType,
} 

impl HumanAgent {
    pub fn new(agent_type: AgentType) -> Self {
        Self { agent_type }
    }
}

use proconio::input;

impl AgentTrait for HumanAgent {
    fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    fn next(&self, _: &Board) -> (usize, usize) {
        println!(
            "次は{:?}の番です",
            self.agent_type
        );
        loop {
            println!("x yを入力してください");
            input! {
                x: usize,
                y: usize,
            }
            if x >= 3 || y >= 3 {
                println!("xとyは0から2の間で指定してください");
                continue;
            }
            return (x, y);
        }
    }

}

pub struct RandomAgent {
    pub agent_type: AgentType,
}

impl RandomAgent {
    pub fn new(agent_type: AgentType) -> Self {
        Self { agent_type }
    }
}

impl AgentTrait for RandomAgent {
    fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    fn next(&self, board: &Board) -> (usize, usize) {
        loop {
            let x = rand::random::<usize>() % 3;
            let y = rand::random::<usize>() % 3;
            if board.get(x,y) == CellState::Empty {
                return (x, y);
            }
        }
    }
}

