use crate::agent::AgentType;
use crate::agent::AgentTrait;
use anyhow::Result;

pub struct Environment {
    board: Board,
    agents: [Box<dyn AgentTrait>; 2],
    curent_agent_index: usize,
}

// 3 * 3 の盤面を表す
#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    cells: [[CellState; 3]; 3],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum CellState {
    Empty,
    Maru(u8),
    Batsu(u8),
    MaruAfterImage,
    BatsuAfterImage,
}

impl Environment {
    pub fn new(
        agent1: Box<dyn AgentTrait>,
        agent2: Box<dyn AgentTrait>,
    ) -> Self {
        let board = Board::new();
        if agent1.agent_type() == agent2.agent_type() {
            panic!("agent1とagent2は異なるagent_typeを持つ必要があります");
        }
        Self {
            board,
            agents: [agent1, agent2],
            curent_agent_index: 0,
        }
    }

    pub fn next(&mut self) -> Result<()> {
        let current_agent = &self.agents[self.curent_agent_index];
        let (x, y) = current_agent.next(&self.board);
        if self.board.cells[y][x] != CellState::Empty {
            return Err(anyhow::anyhow!("すでに置かれています"));
        }
        self.board.put(x, y, self.agents[self.curent_agent_index].agent_type());
        self.curent_agent_index = 1 - self.curent_agent_index;
        self.board.update(self.agents[self.curent_agent_index].agent_type());
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.board.winner().is_some()
    }

    pub fn winner(&self) -> AgentType {
        self.board.winner().unwrap()
    }
    pub fn current_agent_type(&self) -> AgentType {
        self.agents[self.curent_agent_index].agent_type()
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "+---+")?;
        for row in self.board.cells.iter() {
            write!(f, "|")?;
            for cell in row.iter() {
                match cell {
                    CellState::Empty => write!(f, " ")?,
                    CellState::Maru(_) => write!(f, "o")?,
                    CellState::Batsu(_) => write!(f, "x")?,
                    CellState::MaruAfterImage => write!(f, "O")?,
                    CellState::BatsuAfterImage => write!(f, "X")?,
                }
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        writeln!(f, "+---+")?;
        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[CellState::new(); 3]; 3],
        }
    }

    pub(crate) fn get(&self, x: usize, y: usize) -> CellState {
        self.cells[y][x]
    }

    fn put(&mut self, x: usize, y: usize, agent: AgentType) {
        match agent {
            AgentType::MARU => self.cells[y][x] = CellState::Maru(0),
            AgentType::BATSU => self.cells[y][x] = CellState::Batsu(0),
        }
    }

    fn update(&mut self, current_agent: AgentType) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.update(current_agent);
            }
        }
    }

    fn winner(&self) -> Option<AgentType> {
        // check maru batsu
        // check horizontal
        for row in self.cells.iter() {
            if row.iter().all(|cell| cell.is_valid_maru()) {
                return Some(AgentType::MARU);
            }
            if row.iter().all(|cell| cell.is_valid_batsu()) {
                return Some(AgentType::BATSU);
            }
        }

        // check vertical
        for x in 0..3 {
            if (0..3).all(|y| self.cells[y][x].is_valid_maru()) {
                return Some(AgentType::MARU);
            }
            if (0..3).all(|y| self.cells[y][x].is_valid_batsu()) {
                return Some(AgentType::BATSU);
            }
        }

        // check diagonal
        let diagonal = [[(0, 0), (1, 1), (2, 2)], [(0, 2), (1, 1), (2, 0)]];
        for diagonal in diagonal.iter() {
            if diagonal
                .iter()
                .all(|(x, y)| self.cells[*y][*x].is_valid_maru())
            {
                return Some(AgentType::MARU);
            }
            if diagonal
                .iter()
                .all(|(x, y)| self.cells[*y][*x].is_valid_batsu())
            {
                return Some(AgentType::BATSU);
            }
        }
        None
    }
}

impl CellState {
    const DISABLED_TIME: u8 = 3;

    fn new() -> Self {
        Self::Empty
    }

    fn update(&mut self, current_agent: AgentType) {
        match self {
            Self::Maru(counter) => {
                if current_agent == AgentType::MARU {
                    *counter += 1;
                    if *counter == Self::DISABLED_TIME {
                        *self = Self::MaruAfterImage;
                    }
                }
            }
            Self::Batsu(counter) => {
                if current_agent == AgentType::BATSU {
                    *counter += 1;
                    if *counter == Self::DISABLED_TIME {
                        *self = Self::BatsuAfterImage;
                    }
                }
            }
            _ => {
                *self = Self::Empty;
            }
        }
    }

    fn is_valid_maru(&self) -> bool {
        match self {
            Self::Maru(_) => true,
            _ => false,
        }
    }
    fn is_valid_batsu(&self) -> bool {
        match self {
            Self::Batsu(_) => true,
            _ => false,
        }
    }
}
