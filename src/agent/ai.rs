use crate::agent::AgentTrait;
use crate::agent::AgentType;
use crate::agent::Command;
use crate::environment::Board;
use crate::environment::CellState;

pub struct AIAgent {
    agent_type: AgentType,
}

impl AIAgent {
    pub fn new(agent_type: AgentType) -> Self {
        Self { agent_type }
    }

    #[allow(dead_code)]
    fn make_file(&self) {
        // make ai.bin
        std::fs::write("ai.bin", "").unwrap();
    }
}

impl AgentTrait for AIAgent {
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
