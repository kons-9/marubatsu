use crate::agent::Agent;
use anyhow::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct Environment {
    board: Board,
    curent_agent: Agent,
}

// 3 * 3 の盤面を表す
#[derive(Clone, PartialEq, Debug)]
struct Board {
    cells: [[CellState; 3]; 3],
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellState {
    Empty,
    Maru(u8),
    Batsu(u8),
    MaruAfterImage,
    BatsuAfterImage,
}

impl Environment {
    pub fn new() -> Self {
        let board = Board::new();
        let curent_agent = Agent::MARU;
        Self {
            board,
            curent_agent,
        }
    }

    pub fn next(&mut self, x: usize, y: usize) -> Result<()> {
        if self.board.cells[y][x] != CellState::Empty {
            return Err(anyhow::anyhow!("すでに置かれています"));
        }
        self.board.put(x, y, self.curent_agent);
        self.curent_agent = self.curent_agent.next();
        self.board.update(self.curent_agent);
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.board.winner().is_some()
    }

    pub fn winner(&self) -> Agent {
        self.board.winner().unwrap()
    }
    pub fn current_agent(&self) -> Agent {
        self.curent_agent
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

    fn put(&mut self, x: usize, y: usize, agent: Agent) {
        match agent {
            Agent::MARU => self.cells[y][x] = CellState::Maru(0),
            Agent::BATSU => self.cells[y][x] = CellState::Batsu(0),
        }
    }

    fn update(&mut self, current_agent: Agent) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.update(current_agent);
            }
        }
    }

    fn winner(&self) -> Option<Agent> {
        // check maru batsu
        // check horizontal
        for row in self.cells.iter() {
            if row.iter().all(|cell| cell.is_valid_maru()) {
                return Some(Agent::MARU);
            }
            if row.iter().all(|cell| cell.is_valid_batsu()) {
                return Some(Agent::BATSU);
            }
        }

        // check vertical
        for x in 0..3 {
            if (0..3).all(|y| self.cells[y][x].is_valid_maru()) {
                return Some(Agent::MARU);
            }
            if (0..3).all(|y| self.cells[y][x].is_valid_batsu()) {
                return Some(Agent::BATSU);
            }
        }

        // check diagonal
        let diagonal = [[(0, 0), (1, 1), (2, 2)], [(0, 2), (1, 1), (2, 0)]];
        for diagonal in diagonal.iter() {
            if diagonal
                .iter()
                .all(|(x, y)| self.cells[*y][*x].is_valid_maru())
            {
                return Some(Agent::MARU);
            }
            if diagonal
                .iter()
                .all(|(x, y)| self.cells[*y][*x].is_valid_batsu())
            {
                return Some(Agent::BATSU);
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

    fn update(&mut self, current_agent: Agent) {
        match self {
            Self::Maru(counter) => {
                if current_agent == Agent::MARU {
                    *counter += 1;
                    if *counter == Self::DISABLED_TIME {
                        *self = Self::MaruAfterImage;
                    }
                }
            }
            Self::Batsu(counter) => {
                if current_agent == Agent::BATSU {
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
