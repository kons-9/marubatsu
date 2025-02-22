use super::AgentTrait;
use super::AgentType;
use super::Command;
use crate::environment::Board;
use crate::environment::CellState;

pub struct RandomAgent {
    agent_type: AgentType,
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

    fn next(&self, board: &Board) -> Command {
        loop {
            let x = rand::random::<usize>() % 3;
            let y = rand::random::<usize>() % 3;
            if board.get(x, y) == CellState::Empty {
                return Command::Next(x, y);
            }
        }
    }
}
